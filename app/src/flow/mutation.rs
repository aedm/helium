use crate::core::core_mutation::{CoreMutation, SetSlotConnectionsCoreMutation};
use crate::core::node::CoreNodeRef;
use crate::core::slot::CoreSlotConnection;
use crate::flow::dom::Dom;
use crate::flow::flow_node::{FlowSlotIndex, FlowNode, FlowNodeRef, FlowSlot};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use std::collections::HashSet;

pub struct FlowMutationStepResult {
    core_mutation: Option<Box<dyn CoreMutation>>,
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
            core_mutation: None,
            changed_slots: vec![],
        }
    }
}

pub struct SetSlotConnectionsFlowMutation {
    pub node: FlowNodeRef,
    pub slot_index: usize,
    pub connections: Vec<FlowSlotIndex>,
}

impl FlowMutationStep for SetSlotConnectionsFlowMutation {
    fn run(&self, dom: &mut Dom) -> FlowMutationStepResult {
        let mut node = (*self.node).borrow_mut();

        // Change shadow DOM
        node.slots[self.slot_index].connections = self.connections.to_vec();

        // Generate core mutation
        // TODO: more than 1
        assert_eq!(self.connections.len(), 1);
        let flow_connection = &self.connections[0];

        // TODO: can't borrow core node from this thread
        unimplemented!();
        let core_source_node = node.core_node.borrow();
        let core_source_slot = &core_source_node.slots[self.slot_index];
        let core_target_provider = flow_connection.node.borrow().core_node.borrow().providers
            [flow_connection.slot_index]
            .clone();
        let core_connection = CoreSlotConnection::Single(core_target_provider);
        let core_mutation = SetSlotConnectionsCoreMutation {
            slot: core_source_slot.clone(),
            connection: core_connection,
        };

        FlowMutationStepResult {
            core_mutation: Some(Box::new(core_mutation)),
            changed_slots: vec![FlowSlotIndex {
                node: self.node.clone(),
                slot_index: self.slot_index,
            }],
        }
    }
}
