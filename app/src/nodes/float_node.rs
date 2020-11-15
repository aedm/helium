use crate::core::acell::ACell;
use crate::core::node::NodeInner;
use crate::core::provider::CoreProvider;
use crate::core::slot::CoreSlot;
use crate::providers::float_provider::FloatCoreProvider;
use crate::slots::float_slot::FloatCoreSlot;
use std::any::{Any, TypeId};

pub struct FloatNode {
    pub a: FloatCoreSlot,
    pub out: FloatCoreProvider,
}

impl NodeInner for FloatNode {
    fn new() -> FloatNode {
        FloatNode {
            a: FloatCoreSlot::new("a"),
            out: FloatCoreProvider::new("value"),
        }
    }

    fn get_slots(self: &Self) -> Vec<ACell<CoreSlot>> {
        vec![self.a.slot.clone()]
    }

    fn get_providers(self: &Self) -> Vec<ACell<CoreProvider>> {
        vec![self.out.provider.clone()]
    }

    fn run(self: &mut Self) {
        self.out.set(self.a.get());
    }

    fn type_id(&self) -> TypeId {
        println!("inner floatnode type: {:?}", TypeId::of::<FloatNode>());
        TypeId::of::<FloatNode>()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_name(&self) -> &'static str {
        "float"
    }
}
