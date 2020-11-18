use crate::core::node::{CoreNode, CoreNodeInner, NodeId};
use crate::providers::float_provider::FloatCoreProvider;
use crate::slots::float_slot::FloatCoreSlot;

use std::fmt;

pub struct FloatNode {
    inner: CoreNodeInner,
    pub a: FloatCoreSlot,
    pub out: FloatCoreProvider,
}

impl CoreNode for FloatNode {
    fn new(id: NodeId) -> FloatNode {
        let a = FloatCoreSlot::new("a");
        let out = FloatCoreProvider::new("value");
        let slots = vec![a.slot.clone()];
        let providers = vec![out.provider.clone()];
        FloatNode {
            inner: CoreNodeInner::new(id, "float", slots, providers),
            a,
            out,
        }
    }

    fn get_inner(&self) -> &CoreNodeInner {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut CoreNodeInner {
        &mut self.inner
    }

    fn run(self: &mut Self) {
        self.out.set(self.a.get());
    }
}

impl fmt::Debug for FloatNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
