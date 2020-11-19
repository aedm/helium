use crate::core::node::CoreNode;

use crate::core::core_node_descriptor::{CoreNodeDescriptor, NodeId};
use crate::slots::node_slot::NodeCoreSlot;
use std::fmt;

pub struct CoreRootNode {
    inner: CoreNodeDescriptor,
    pub slot: NodeCoreSlot,
}

impl CoreNode for CoreRootNode {
    fn new(id: NodeId) -> CoreRootNode {
        let slot = NodeCoreSlot::new("all_nodes");
        let slots = vec![slot.slot.clone()];
        let providers = vec![];
        CoreRootNode {
            inner: CoreNodeDescriptor::new(id, "root", slots, providers),
            slot,
        }
    }

    fn descriptor(&self) -> &CoreNodeDescriptor {
        &self.inner
    }

    fn descriptor_mut(&mut self) -> &mut CoreNodeDescriptor {
        &mut self.inner
    }

    fn run(&mut self) {}
}

impl fmt::Debug for CoreRootNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
