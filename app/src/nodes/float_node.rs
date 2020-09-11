use crate::flow::slot::{FloatSlot, Slot};
use crate::flow::provider::{FloatProvider, Provider};
use crate::flow::node::NodeInner;
use crate::flow::rf::Rf;

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
