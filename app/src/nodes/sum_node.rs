use crate::core::node::NodeInner;
use crate::core::provider::{CoreProvider, FloatCoreProvider};
use crate::core::rf::Rf;
use crate::core::slot::{CoreSlot, FloatCoreSlot};

pub struct SumNode {
    pub a: FloatCoreSlot,
    pub b: FloatCoreSlot,
    pub sum: FloatCoreProvider,
}

impl NodeInner for SumNode {
    fn new() -> SumNode {
        SumNode {
            a: FloatCoreSlot::new("A"),
            b: FloatCoreSlot::new("B"),
            sum: FloatCoreProvider::new("Sum"),
        }
    }

    fn get_slots(self: &Self) -> Vec<Rf<CoreSlot>> {
        vec![self.a.slot.clone(), self.b.slot.clone()]
    }

    fn get_providers(self: &Self) -> Vec<Rf<CoreProvider>> {
        vec![self.sum.provider.clone()]
    }

    fn run(self: &mut Self) {
        let result = self.a.get() + self.b.get();
        self.sum.set(result);
    }
}
