use crate::core::node::{CoreNodeRef, NodeId};
use crate::flow::flow_node::FlowNodeRef;
use std::collections::HashMap;

pub struct Dom {
    flow_nodes: HashMap<NodeId, FlowNodeRef>,
    // core_nodes: HashMap<NodeId, CoreNodeRef>,
}

impl Dom {
    pub fn new() -> Dom {
        Dom {
            flow_nodes: Default::default(),
            // core_nodes: Default::default(),
        }
    }

    pub fn add_flow_node(&mut self, flow_node: &FlowNodeRef) {
        self.flow_nodes
            .insert(flow_node.borrow().id, flow_node.clone());
    }

    pub fn remove_flow_node(&mut self, node_id: NodeId) {
        self.flow_nodes.remove(&node_id);
    }
}
