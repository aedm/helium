use crate::node_descriptor::{NodeDescriptor, NodeId};
use crate::node::Node;
use crate::slots::node_slot::NodeSlot;
use std::fmt;

pub struct RootNode {
    inner: NodeDescriptor,
    pub slot: NodeSlot,
}

impl Node for RootNode {
    fn new(id: NodeId) -> RootNode {
        let slot = NodeSlot::new("all_nodes");
        let slots = vec![slot.slot.clone()];
        let providers = vec![];
        RootNode {
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

impl fmt::Debug for RootNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
