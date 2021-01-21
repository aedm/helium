use crate::render_graph::mutation::{
    Mutation, MutationSequence, SetNodeDependencyListParams, SetSlotConnectionsParams,
};
use crate::render_graph::node::{NodeProviderRef, NodeSlotRef};
use crate::dom::dom::Dom;
use crate::dom::element::{DomElementRef, DomSlotRef, DomProviderRef};
use crate::dom::topological_order::TopologicalOrder;
use std::collections::HashSet;
use crate::dom::transaction_add_element::TransactionAddElement;
use crate::dom::transaction_remove_element::TransactionRemoveElement;
use crate::dom::transaction_set_slot_connections::TransactionSetSlotConnections;

pub struct TransactionStepResult {
    pub changed_slots: Vec<DomSlotRef>,
    pub graph_mutations: Vec<Mutation>,
}

pub trait TransactionStep {
    fn run(&self, dom: &mut Dom) -> TransactionStepResult;
}

pub struct Transaction {
    pub steps: Vec<Box<dyn TransactionStep>>,
    pub changed_slots: HashSet<DomSlotRef>,
}

impl Transaction {
    pub fn new(steps: Vec<Box<dyn TransactionStep>>) -> Transaction {
        Transaction {
            steps,
            changed_slots: HashSet::new(),
        }
    }

    pub fn add_element(element: &DomElementRef) -> Box<TransactionAddElement> {
        TransactionAddElement::new(element)
    }

    pub fn remove_element(element: &DomElementRef) -> Box<TransactionRemoveElement> {
        TransactionRemoveElement::new(element)
    }

    pub fn set_slot_connections(
        slot: DomSlotRef,
        connections: Vec<DomProviderRef>,
    ) -> Box<TransactionSetSlotConnections> {
        TransactionSetSlotConnections::new(slot, connections)
    }

    pub fn run(&mut self, dom: &mut Dom) -> MutationSequence {
        let mut direct_core_mutations: Vec<Mutation> = Vec::new();
        for step in &mut self.steps {
            let result = step.run(dom);
            for changed_slot in &result.changed_slots {
                self.changed_slots.insert(changed_slot.clone());
            }
            direct_core_mutations.extend(result.graph_mutations);
        }
        self.create_core_mutations(direct_core_mutations)
    }

    fn create_core_mutations(&self, mut steps: Vec<Mutation>) -> MutationSequence {
        for flow_slot_index in &self.changed_slots {
            let flow_slot = &flow_slot_index.element.borrow().slots[flow_slot_index.slot_index];
            let connection: Vec<_> = flow_slot
                .connections
                .iter()
                .map(|x| NodeProviderRef {
                    node: x.element.borrow().core_node.clone(),
                    provider_index: x.provider_index,
                })
                .collect();
            let item_count = connection.len();
            let core_mutation = Mutation::SetSlotConnections(SetSlotConnectionsParams {
                slot: NodeSlotRef {
                    node: flow_slot_index.element.borrow().core_node.clone(),
                    slot_index: flow_slot_index.slot_index,
                },
                connection,
                swap_vector: Vec::with_capacity(item_count),
            });
            steps.push(core_mutation);
        }
        let mut set: HashSet<DomElementRef> = HashSet::new();
        for flow_slot_index in &self.changed_slots {
            collect_affected_dependencies(&flow_slot_index.element, &mut set);
        }
        for flow_node in &set {
            let mut flow_dependencies = TopologicalOrder::generate(flow_node);
            // removes self from dependencies
            flow_dependencies.pop();
            let dependency_list = flow_dependencies
                .iter()
                .map(|x| x.borrow().core_node.clone())
                .collect();
            let core_mutation = Mutation::SetNodeDependencyList(SetNodeDependencyListParams {
                node: flow_node.borrow().core_node.clone(),
                dependency_list,
            });
            steps.push(core_mutation);
        }
        MutationSequence::new(steps)
    }
}

// Adds all nodes that depend on @node to @set.
fn collect_affected_dependencies(node: &DomElementRef, set: &mut HashSet<DomElementRef>) {
    if set.contains(node) {
        return;
    }
    set.insert(node.clone());
    for provider in &node.borrow().providers {
        for connection in &provider.connections {
            collect_affected_dependencies(&connection.element, set);
        }
    }
}
