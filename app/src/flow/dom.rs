use crate::core::node::NodeId;
use crate::flow::flow_node::{FlowNodeRef, FlowNode};
use std::collections::HashMap;
use crate::core::core_dom::CoreDom;

pub struct FlowDom {
    flow_nodes: HashMap<NodeId, FlowNodeRef>,
    flow_root: FlowNodeRef,
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

    // pub fn remove_flow_node(&mut self, node_id: NodeId) {
    //     self.flow_nodes.remove(&node_id);
    // }
}
