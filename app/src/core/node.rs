use crate::core::provider::CoreProvider;
use crate::core::rf::ACell;
use crate::core::slot::CoreSlot;
use std::any::{Any, TypeId};
use std::borrow::Borrow;

pub type CoreNodeRef = ACell<CoreNode>;
pub type NodeId = u64;

pub struct CoreNode {
    pub slots: Vec<ACell<CoreSlot>>,
    pub providers: Vec<ACell<CoreProvider>>,
    pub inner: Box<dyn NodeInner>,
    pub dependency_list: Vec<CoreNodeRef>,
}

pub enum NodeType {
    _Custom,
    Float,
    Sum,
}

pub trait NodeInner {
    fn new() -> Self
    where
        Self: std::marker::Sized;
    fn get_slots(&self) -> Vec<ACell<CoreSlot>> { vec![] }
    fn get_providers(&self) -> Vec<ACell<CoreProvider>> { vec![] }
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
        {
            let rf_mut = &rf.borrow_mut();
            for provider in &rf_mut.providers {
                provider.borrow_mut().owner = rf.downgrade();
            }
            for slot in &rf_mut.slots {
                slot.borrow_mut().owner = rf.downgrade();
            }
        }
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

    pub fn inner_type_id(&self) -> TypeId {
        (*self.inner).type_id()
    }
}