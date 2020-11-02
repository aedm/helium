use crate::core::core_mutation::{CoreMutation, CoreMutationSequence, SetSlotConnectionsCoreMutation, SetNodeDependencyListCoreMutation};
use crate::core::node::{CoreNodeRef, CoreProviderIndex, CoreSlotIndex};
use crate::flow::dom::Dom;
use crate::flow::flow_node::{FlowNode, FlowNodeRef, FlowSlot, FlowSlotIndex};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashSet;
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
use crate::flow::topological_order::TopologicalOrder;

pub struct FlowMutationStepResult {
    pub changed_slots: Vec<FlowSlotIndex>,
}

pub trait FlowMutationStep {
    fn run(&self, dom: &mut Dom) -> FlowMutationStepResult;
}

pub struct FlowMutation {
    pub steps: Vec<Box<dyn FlowMutationStep>>,
    pub changed_slots: HashSet<FlowSlotIndex>,
}

impl FlowMutation {
    pub fn new(steps: Vec<Box<dyn FlowMutationStep>>) -> FlowMutation {
        FlowMutation {
            steps,
            changed_slots: HashSet::new(),
        }
    }

    pub fn run(&mut self, dom: &mut Dom) -> CoreMutationSequence {
        for step in &mut self.steps {
            let result = step.run(dom);
            for changed_slot in &result.changed_slots {
                self.changed_slots.insert(changed_slot.clone());
            }
        }
        self.create_core_mutations()
    }

    fn create_core_mutations(&self) -> CoreMutationSequence {
        let mut steps = Vec::<Box<dyn CoreMutation>>::new();
        println!("changed_slots: {:?}", &self.changed_slots.len());
        for flow_slot_index in &self.changed_slots {
            let flow_slot = &flow_slot_index.node.borrow().slots[flow_slot_index.slot_index];
            let connection: Vec<_> = flow_slot
                .connections
                .iter()
                .map(|x| CoreProviderIndex {
                    node: x.node.borrow().core_node.clone(),
                    provider_index: x.slot_index,
                })
                .collect();
            let item_count = connection.len();
            let core_mutation = SetSlotConnectionsCoreMutation {
                slot: CoreSlotIndex {
                    node: flow_slot_index.node.borrow().core_node.clone(),
                    slot_index: flow_slot_index.slot_index,
                },
                connection,
                swap_vector: Vec::with_capacity(item_count),
            };
            steps.push(Box::new(core_mutation));
        }
        let mut set: HashSet<FlowNodeRef> = HashSet::new();
        for flow_slot_index in &self.changed_slots {
            collect_affected_dependencies(&flow_slot_index.node, &mut set);
        }
        for flow_node in &set {
            let flow_dependencies = TopologicalOrder::generate(flow_node);
            let dependency_list =
                flow_dependencies.iter().map(|x| x.borrow().core_node.clone()).collect();
            let core_mutation = SetNodeDependencyListCoreMutation {
                node: flow_node.borrow().core_node.clone(),
                dependency_list,
            };
            steps.push(Box::new(core_mutation));
        }
        CoreMutationSequence { steps }
    }
}

// Adds all nodes that depend on @node to @set.
fn collect_affected_dependencies(node: &FlowNodeRef, set: &mut HashSet<FlowNodeRef>) {
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
