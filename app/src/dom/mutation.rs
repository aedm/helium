use crate::render_graph::graph_mutation::{
    GraphMutation, GraphMutationSequence, SetNodeDependencyListParams, SetSlotConnectionsParams,
};
use crate::render_graph::node::{NodeProviderRef, NodeSlotRef};
use crate::dom::dom::Dom;
use crate::dom::dom_element::{DomElementRef, DomSlotRef};
use crate::dom::topological_order::TopologicalOrder;
use std::collections::HashSet;

pub struct DomMutationStepResult {
    pub changed_slots: Vec<DomSlotRef>,
    pub core_mutations: Vec<GraphMutation>,
}

pub trait DomMutationStep {
    fn run(&self, dom: &mut Dom) -> DomMutationStepResult;
}

pub struct DomMutation {
    pub steps: Vec<Box<dyn DomMutationStep>>,
    pub changed_slots: HashSet<DomSlotRef>,
}

impl DomMutation {
    pub fn new(steps: Vec<Box<dyn DomMutationStep>>) -> DomMutation {
        DomMutation {
            steps,
            changed_slots: HashSet::new(),
        }
    }

    pub fn run(&mut self, dom: &mut Dom) -> GraphMutationSequence {
        let mut direct_core_mutations: Vec<GraphMutation> = Vec::new();
        for step in &mut self.steps {
            let result = step.run(dom);
            for changed_slot in &result.changed_slots {
                self.changed_slots.insert(changed_slot.clone());
            }
            direct_core_mutations.extend(result.core_mutations);
        }
        self.create_core_mutations(direct_core_mutations)
    }

    fn create_core_mutations(&self, mut steps: Vec<GraphMutation>) -> GraphMutationSequence {
        for flow_slot_index in &self.changed_slots {
            let flow_slot = &flow_slot_index.node.borrow().slots[flow_slot_index.slot_index];
            let connection: Vec<_> = flow_slot
                .connections
                .iter()
                .map(|x| NodeProviderRef {
                    node: x.node.borrow().core_node.clone(),
                    provider_index: x.provider_index,
                })
                .collect();
            let item_count = connection.len();
            let core_mutation = GraphMutation::SetSlotConnections(SetSlotConnectionsParams {
                slot: NodeSlotRef {
                    node: flow_slot_index.node.borrow().core_node.clone(),
                    slot_index: flow_slot_index.slot_index,
                },
                connection,
                swap_vector: Vec::with_capacity(item_count),
            });
            steps.push(core_mutation);
        }
        let mut set: HashSet<DomElementRef> = HashSet::new();
        for flow_slot_index in &self.changed_slots {
            collect_affected_dependencies(&flow_slot_index.node, &mut set);
        }
        for flow_node in &set {
            let mut flow_dependencies = TopologicalOrder::generate(flow_node);
            // removes self from dependencies
            flow_dependencies.pop();
            let dependency_list = flow_dependencies
                .iter()
                .map(|x| x.borrow().core_node.clone())
                .collect();
            let core_mutation = GraphMutation::SetNodeDependencyList(SetNodeDependencyListParams {
                node: flow_node.borrow().core_node.clone(),
                dependency_list,
            });
            steps.push(core_mutation);
        }
        GraphMutationSequence::new(steps)
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
            collect_affected_dependencies(&connection.node, set);
        }
    }
}
