use crate::core::acell::ACell;
use crate::core::node::NodeInner;
use crate::core::slot::CoreSlot;
use crate::slots::float_slot::FloatCoreSlot;
use std::any::{Any, TypeId};

pub struct CoreRootNode {
    pub slot: FloatCoreSlot,
}

impl NodeInner for CoreRootNode {
    fn new() -> CoreRootNode {
        CoreRootNode {
            slot: FloatCoreSlot::new("all_nodes"),
        }
    }

    fn get_slots(self: &Self) -> Vec<ACell<CoreSlot>> {
        vec![self.slot.slot.clone()]
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<CoreRootNode>()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type_name(&self) -> &'static str {
        "root"
    }
}
