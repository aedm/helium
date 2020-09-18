use crate::core::node::NodeInner;
use crate::core::provider::{FloatProvider, Provider};
use crate::core::rf::Rf;
use crate::core::slot::{FloatSlot, Slot};

pub struct SumNode {
    pub a: FloatSlot,
    pub b: FloatSlot,
    pub sum: FloatProvider,
}

impl NodeInner for SumNode {
    fn new() -> SumNode {
        SumNode {
            a: FloatSlot::new("A"),
            b: FloatSlot::new("B"),
            sum: FloatProvider::new("Sum"),
        }
    }

    fn get_slots(self: &Self) -> Vec<Rf<Slot>> {
        vec![self.a.slot.clone(), self.b.slot.clone()]
    }

    fn get_providers(self: &Self) -> Vec<Rf<Provider>> {
        vec![self.sum.provider.clone()]
    }

    fn run(self: &mut Self) {
        let result = self.a.get() + self.b.get();
        self.sum.set(result);
    }
}
