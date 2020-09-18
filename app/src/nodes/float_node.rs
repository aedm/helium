use crate::core::node::NodeInner;
use crate::core::provider::{FloatProvider, Provider};
use crate::core::rf::Rf;
use crate::core::slot::{FloatSlot, Slot};

pub struct FloatNode {
    pub a: FloatSlot,
    pub out: FloatProvider,
}

impl NodeInner for FloatNode {
    fn new() -> FloatNode {
        FloatNode {
            a: FloatSlot::new("A"),
            out: FloatProvider::new("Value"),
        }
    }

    fn get_slots(self: &Self) -> Vec<Rf<Slot>> {
        vec![self.a.slot.clone()]
    }

    fn get_providers(self: &Self) -> Vec<Rf<Provider>> {
        vec![self.out.provider.clone()]
    }

    fn run(self: &mut Self) {
        self.out.set(self.a.get());
    }
}
