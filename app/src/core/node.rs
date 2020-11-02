use crate::core::acell::ACell;
use crate::core::provider::CoreProvider;
use crate::core::slot::CoreSlot;
use std::any::{Any, TypeId};

pub type CoreNodeRef = ACell<CoreNode>;
pub type NodeId = u64;

pub struct CoreNode {
    pub slots: Vec<ACell<CoreSlot>>,
    pub providers: Vec<ACell<CoreProvider>>,
    pub inner: Box<dyn NodeInner>,
    pub dependency_list: Vec<CoreNodeRef>,
}

// pub enum NodeType {
//     _Custom,
//     Float,
//     Sum,
// }

pub struct CoreProviderIndex {
    pub node: CoreNodeRef,
    pub provider_index: usize,
}

pub struct CoreSlotIndex {
    pub node: CoreNodeRef,
    pub slot_index: usize,
}

pub trait NodeInner {
    fn new() -> Self
    where
        Self: std::marker::Sized;
    fn get_slots(&self) -> Vec<ACell<CoreSlot>> {
        vec![]
    }
    fn get_providers(&self) -> Vec<ACell<CoreProvider>> {
        vec![]
    }
    fn run(&mut self) {}
    fn type_id(&self) -> TypeId;
    fn as_any(&self) -> &dyn Any;
}

impl CoreNode {
    pub fn new<T: 'static + NodeInner>() -> CoreNodeRef {
        let inner = Box::new(T::new());
        let rf = ACell::new(CoreNode {
            dependency_list: vec![],
            slots: inner.get_slots(),
            providers: inner.get_providers(),
            inner,
        });
        rf
    }

    pub fn run(&mut self) {
        self.inner.run();
    }

    pub fn run_deps(&mut self) {
        for dep in &self.dependency_list {
            dep.borrow_mut().run();
        }
        self.run();
    }

    // pub fn inner_type_id(&self) -> TypeId {
    //     (*self.inner).type_id()
    // }
}
