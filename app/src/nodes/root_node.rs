use crate::core::acell::ACell;
use crate::core::node::NodeInner;
use crate::core::provider::{CoreProvider, FloatCoreProvider};
use crate::core::slot::{CoreSlot, FloatCoreSlot};
use std::any::{Any, TypeId};

pub struct CoreRootNode {}

impl NodeInner for CoreRootNode {
    fn new() -> CoreRootNode {
        CoreRootNode {}
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<CoreRootNode>()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
