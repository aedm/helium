use crate::node_descriptor::{NodeDescriptor, NodeId};
use crate::node::Node;
use crate::providers::float_provider::FloatProvider;
use crate::slots::float_slot::FloatSlot;
use std::fmt;

pub struct FloatNode {
    inner: NodeDescriptor,
    pub a: FloatSlot,
    pub out: FloatProvider,
}

impl Node for FloatNode {
    fn new(id: NodeId) -> FloatNode {
        let a = FloatSlot::new("a");
        let out = FloatProvider::new("value");
        let slots = vec![a.slot.clone()];
        let providers = vec![out.provider.clone()];
        FloatNode {
            inner: NodeDescriptor::new(id, "float", slots, providers),
            a,
            out,
        }
    }

    fn descriptor(&self) -> &NodeDescriptor {
        &self.inner
    }

    fn descriptor_mut(&mut self) -> &mut NodeDescriptor {
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
