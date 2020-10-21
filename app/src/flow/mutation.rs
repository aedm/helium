use crate::core::core_mutation::{CoreMutation, SetSlotConnectionsCoreMutation};
use crate::core::node::CoreNodeRef;
use crate::flow::dom::Dom;
use crate::flow::flow_node::{FlowNode, FlowNodeRef, FlowSlot, FlowSlotIndex};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashSet;
use std::mem;
use std::rc::Rc;

pub struct FlowMutationStepResult {
    changed_slots: Vec<FlowSlotIndex>,
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

    pub fn run(&mut self, dom: &mut Dom) {
        for step in &mut self.steps {
            step.run(dom);
        }
    }
}

pub struct CreateNodeFlowMutation {
    pub new_node: FlowNodeRef,
}

impl FlowMutationStep for CreateNodeFlowMutation {
    fn run(&self, dom: &mut Dom) -> FlowMutationStepResult {
        dom.add_flow_node(&self.new_node);
        FlowMutationStepResult {
            changed_slots: vec![],
        }
    }
}

pub struct SetSlotConnectionsFlowMutation {
    pub node_slot: FlowSlotIndex,
    pub connections: Vec<FlowSlotIndex>,
}

impl FlowMutationStep for SetSlotConnectionsFlowMutation {
    fn run(&self, dom: &mut Dom) -> FlowMutationStepResult {
        // Change shadow DOM
        let mut node = (*self.node_slot.node).borrow_mut();
        node.slots[self.node_slot.slot_index].connections = self.connections.to_vec();

        // Generate core mutation
        FlowMutationStepResult {
            changed_slots: vec![self.node_slot.clone()],
        }
    }
}
