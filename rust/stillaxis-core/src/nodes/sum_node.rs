use std::fmt;
use crate::providers::float_provider::FloatCoreProvider;
use crate::slots::float_slot::FloatCoreSlot;
use crate::core_node_descriptor::{CoreNodeDescriptor, NodeId};
use crate::node::CoreNode;

pub struct SumNode {
    inner: CoreNodeDescriptor,
    pub a: FloatCoreSlot,
    pub b: FloatCoreSlot,
    pub sum: FloatCoreProvider,
}

impl CoreNode for SumNode {
    fn new(id: NodeId) -> SumNode {
        let a = FloatCoreSlot::new("a");
        let b = FloatCoreSlot::new("b");
        let sum = FloatCoreProvider::new("sum");
        let slots = vec![a.slot.clone(), b.slot.clone()];
        let providers = vec![sum.provider.clone()];
        SumNode {
            inner: CoreNodeDescriptor::new(id, "sum_floats", slots, providers),
            a,
            b,
            sum,
        }
    }

    fn descriptor(&self) -> &CoreNodeDescriptor {
        &self.inner
    }

    fn descriptor_mut(&mut self) -> &mut CoreNodeDescriptor {
        &mut self.inner
    }

    fn run(self: &mut Self) {
        let result = self.a.get() + self.b.get();
        self.sum.set(result);
    }
}

impl fmt::Debug for SumNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
