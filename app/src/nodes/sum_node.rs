use crate::core::acell::ACell;
use crate::core::provider::CoreProvider;
use crate::core::slot::CoreSlot;
use crate::providers::float_provider::FloatCoreProvider;
use crate::slots::float_slot::FloatCoreSlot;
use std::any::{Any, TypeId};
use crate::core::node::{CoreNodeInner, NodeId, CoreNode};
use std::fmt;

pub struct SumNode {
    inner: CoreNodeInner,
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
            inner: CoreNodeInner::new(id, "sum_floats", slots, providers),
            a,
            b,
            sum,
        }
    }

    fn get_inner(&self) -> &CoreNodeInner {
        &self.inner
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
