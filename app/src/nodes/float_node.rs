use crate::core::node::CoreNode;
use crate::providers::float_provider::FloatCoreProvider;
use crate::slots::float_slot::FloatCoreSlot;

use crate::core::core_node_descriptor::{CoreNodeDescriptor, NodeId};
use std::fmt;

pub struct FloatNode {
    inner: CoreNodeDescriptor,
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
            inner: CoreNodeDescriptor::new(id, "float", slots, providers),
            a,
            out,
        }
    }

    fn descriptor(&self) -> &CoreNodeDescriptor {
        &self.inner
    }

    fn descriptor_mut(&mut self) -> &mut CoreNodeDescriptor {
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
