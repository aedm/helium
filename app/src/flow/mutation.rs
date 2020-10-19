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

pub trait FlowMutationStep {
    fn run(&self, dom: &mut Dom) -> Option<Box<dyn CoreMutation>>;
}

pub struct FlowMutation {
    pub steps: Vec<Box<dyn FlowMutationStep>>,
    pub changed_slots: HashSet<FlowSlotIndex>,
}

impl FlowMutation {
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
    fn run(&self, dom: &mut Dom) -> Option<Box<dyn CoreMutation>> {
        dom.add_flow_node(&self.new_node);
        None
    }
}

pub struct SetSlotConnectionsFlowMutation {
    pub node: FlowNodeRef,
    pub slot_index: usize,
    pub connections: Vec<FlowSlotIndex>,
}

impl FlowMutationStep for SetSlotConnectionsFlowMutation {
    fn run(&self, dom: &mut Dom) -> Option<Box<dyn CoreMutation>> {
        let mut node = (*self.node).borrow_mut();
        node.slots[self.slot_index].connections = self.connections.to_vec();
        assert_eq!(self.connections.len(), 1);
        let flow_connection = &self.connections[0];
        let core_source_node = node.core_node.borrow();
        let core_source_slot = &core_source_node.slots[self.slot_index];
        let core_target_provider = flow_connection.node.borrow().core_node.borrow().providers
            [flow_connection.slot_index]
            .clone();
        let core_connection = CoreSlotConnection::Single(core_target_provider);
        Some(Box::new(SetSlotConnectionsCoreMutation {
            slot: core_source_slot.clone(),
            connection: core_connection,
        }))
    }
}
