use crate::core::acell::ACell;
use crate::core::node::NodeInner;
use crate::core::provider::CoreProvider;
use crate::core::slot::CoreSlot;
use crate::providers::float_provider::FloatCoreProvider;
use crate::slots::float_slot::FloatCoreSlot;
use std::any::{Any, TypeId};

pub struct SumNode {
    pub a: FloatCoreSlot,
    pub b: FloatCoreSlot,
    pub sum: FloatCoreProvider,
}

impl NodeInner for SumNode {
    fn new() -> SumNode {
        SumNode {
            a: FloatCoreSlot::new("a"),
            b: FloatCoreSlot::new("b"),
            sum: FloatCoreProvider::new("sum"),
        }
    }

    fn get_slots(self: &Self) -> Vec<ACell<CoreSlot>> {
        vec![self.a.slot.clone(), self.b.slot.clone()]
    }

    fn get_providers(self: &Self) -> Vec<ACell<CoreProvider>> {
        vec![self.sum.provider.clone()]
    }

    fn run(self: &mut Self) {
        let result = self.a.get() + self.b.get();
        self.sum.set(result);
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<SumNode>()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
