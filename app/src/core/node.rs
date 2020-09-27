use crate::core::provider::CoreProvider;
use crate::core::rf::Rf;
use crate::core::slot::CoreSlot;
use std::sync::atomic::{AtomicU64, Ordering};

pub type CoreNodeRef = Rf<CoreNode>;
pub type NodeId = u64;

pub struct CoreNode {
    pub slots: Vec<Rf<CoreSlot>>,
    pub providers: Vec<Rf<CoreProvider>>,
    inner: Box<dyn NodeInner>,
    pub dependency_list: Vec<CoreNodeRef>,
}

pub trait NodeInner {
    fn new() -> Self
    where
        Self: Sized;
    fn get_slots(self: &Self) -> Vec<Rf<CoreSlot>>;
    fn get_providers(self: &Self) -> Vec<Rf<CoreProvider>>;
    fn run(self: &mut Self);
}

impl CoreNode {
    pub fn new<T: 'static + NodeInner>() -> CoreNodeRef {
        let inner = Box::new(T::new());
        let rf = Rf::new(CoreNode {
            // id: NODE_ID_GENERATOR.fetch_add(1, Ordering::Relaxed),
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
}
