use crate::core::core_dom::CoreDom;
use crate::core::node::NodeId;
use crate::flow::flow_node::{FlowNode, FlowNodeRef};
use std::collections::HashMap;

pub struct FlowDom {
    pub flow_nodes: HashMap<NodeId, FlowNodeRef>,
    pub flow_root: FlowNodeRef,
}

impl FlowDom {
    pub fn new(core_dom: &CoreDom) -> FlowDom {
        FlowDom {
            flow_root: FlowNode::from_core_node(&core_dom.core_root),
            flow_nodes: Default::default(),
        }
    }

    pub fn add_flow_node(&mut self, flow_node: &FlowNodeRef) {
        self.flow_nodes
            .insert(flow_node.borrow().id, flow_node.clone());
    }

    pub fn remove_flow_node(&mut self, flow_node: &FlowNodeRef) {
        self.flow_nodes.remove(&flow_node.borrow().id);
    }
}
