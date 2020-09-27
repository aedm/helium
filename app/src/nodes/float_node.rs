use crate::core::node::NodeInner;
use crate::core::provider::{CoreProvider, FloatCoreProvider};
use crate::core::rf::Rf;
use crate::core::slot::{CoreSlot, FloatCoreSlot};

pub struct FloatNode {
    pub a: FloatCoreSlot,
    pub out: FloatCoreProvider,
}

impl NodeInner for FloatNode {
    fn new() -> FloatNode {
        FloatNode {
            a: FloatCoreSlot::new("A"),
            out: FloatCoreProvider::new("Value"),
        }
    }

    fn get_slots(self: &Self) -> Vec<Rf<CoreSlot>> {
        vec![self.a.slot.clone()]
    }

    fn get_providers(self: &Self) -> Vec<Rf<CoreProvider>> {
        vec![self.out.provider.clone()]
    }

    fn run(self: &mut Self) {
        self.out.set(self.a.get());
    }
}
