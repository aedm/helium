use crate::render_graph::node::Node;

use crate::render_graph::node_descriptor::{NodeDescriptor, NodeId};
use crate::slots::node_slot::NodeCoreSlot;
use std::fmt;

pub struct CoreRootNode {
    inner: NodeDescriptor,
    pub slot: NodeCoreSlot,
}

impl Node for CoreRootNode {
    fn new(id: NodeId) -> CoreRootNode {
        let slot = NodeCoreSlot::new("all_nodes");
        let slots = vec![slot.slot.clone()];
        let providers = vec![];
        CoreRootNode {
            inner: NodeDescriptor::new(id, "root", slots, providers),
            slot,
        }
    }

    fn descriptor(&self) -> &NodeDescriptor {
        &self.inner
    }

    fn descriptor_mut(&mut self) -> &mut NodeDescriptor {
        &mut self.inner
    }

    fn run(&mut self) {}
}

impl fmt::Debug for CoreRootNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
