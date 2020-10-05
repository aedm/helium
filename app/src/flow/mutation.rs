use crate::core::core_mutation::{CoreMutation, SetSlotConnectionsCoreMutation};
use crate::core::node::CoreNodeRef;
use crate::flow::dom::Dom;
use crate::flow::flow_node::{FlowNode, FlowNodeRef, FlowSlot, FlowConnection};
use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::BorrowMut;
use std::mem;
use crate::core::slot::CoreSlotConnection;

pub trait FlowMutation {
    fn run(&self, dom: &mut Dom) -> Option<Box<dyn CoreMutation>>;
}

pub struct FlowMutationSequence {
    pub steps: Vec<Box<dyn FlowMutation>>,
}

impl FlowMutationSequence {
    pub fn run(&mut self, dom: &mut Dom) {
        for step in &mut self.steps {
            step.run(dom);
        }
    }
}

pub struct CreateNodeFlowMutation {
    pub new_node: FlowNodeRef,
}

impl FlowMutation for CreateNodeFlowMutation {
    fn run(&self, dom: &mut Dom) -> Option<Box<dyn CoreMutation>> {
        dom.add_flow_node(&self.new_node);
        None
    }
}

pub struct SetSlotConnectionsFlowMutation {
    pub node: FlowNodeRef,
    pub slot_index: usize,
    pub connections: Vec<FlowConnection>,
}

impl FlowMutation for SetSlotConnectionsFlowMutation {
    fn run(&self, dom: &mut Dom) -> Option<Box<dyn CoreMutation>> {
        let mut node = (*self.node).borrow_mut();
        node.slots[self.slot_index].connections = self.connections.to_vec();
        let core_node = node.core_node.borrow();
        let core_slot = &core_node.slots[self.slot_index];
        unimplemented!()
        // Some(Box::new(SetSlotConnectionsCoreMutation{
        //     slot: core_slot.clone(),
        //     connection: CoreSlotConnection::None
        // }))
    }
}
