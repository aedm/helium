use crate::core::core_mutation::{CoreMutation, SetSlotConnectionsCoreMutation, CoreMutationSequence};
use crate::core::node::{CoreNodeRef, CoreSlotIndex, CoreProviderIndex};
use crate::flow::dom::Dom;
use crate::flow::flow_node::{FlowNode, FlowNodeRef, FlowSlot, FlowSlotIndex};
use std::borrow::{BorrowMut, Borrow};
use std::cell::RefCell;
use std::collections::HashSet;
use std::mem;
use std::rc::Rc;
use std::ops::Deref;

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
        println!("changed_slots: {:?}", &self.changed_slots);
        for flow_slot_index in &self.changed_slots {
            let flow_slot = &flow_slot_index.node.borrow().slots[flow_slot_index.slot_index];
            let connection: Vec<_> = flow_slot.connections.iter()
                .map(|x| CoreProviderIndex {
                    node: x.node.borrow().core_node.clone(),
                    provider_index: x.slot_index })
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
        CoreMutationSequence { steps }
    }
}


