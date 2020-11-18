use crate::core::acell::ACell;
use crate::core::node::{CoreNode, CoreNodeInner, NodeId};
use crate::core::slot::CoreSlot;
use crate::slots::float_slot::FloatCoreSlot;
use std::any::{Any, TypeId};
use std::fmt;

pub struct CoreRootNode {
    inner: CoreNodeInner,
    pub slot: FloatCoreSlot,
}

impl CoreNode for CoreRootNode {
    fn new(id: NodeId) -> CoreRootNode {
        let slot = FloatCoreSlot::new("all_nodes");
        let slots = vec![slot.slot.clone()];
        let providers = vec![];
        CoreRootNode {
            inner: CoreNodeInner::new(id, "root", slots, providers),
            slot,
        }
    }

    fn get_inner(&self) -> &CoreNodeInner {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut CoreNodeInner {
        &mut self.inner
    }

    fn run(&mut self) {
        // unimplemented!()
    }
}

impl fmt::Debug for CoreRootNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
