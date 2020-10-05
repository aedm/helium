use crate::core::core_mutation::CoreMutation;
use crate::core::node::CoreNodeRef;
use crate::flow::dom::Dom;
use crate::flow::flow_node::{FlowNode, FlowNodeRef, FlowSlot, FlowConnection};
use std::cell::RefCell;
use std::rc::Rc;

trait FlowMutation {
    fn run(&self, dom: &mut Dom) -> Option<Box<dyn CoreMutation>>;
}

struct FlowMutationSequence {
    steps: Vec<Box<dyn FlowMutation>>,
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

struct SetSlotConnectionsFlowMutation {
    node: FlowNodeRef,
    slot_index: usize,
    connections: Vec<FlowConnection>,
}

impl FlowMutation for SetSlotConnectionsFlowMutation {
    fn run(&self, dom: &mut Dom) -> Option<Box<dyn CoreMutation>> {
        // self.node.borrow_mut()
        unimplemented!()
    }
}
