use crate::core::provider::Provider;
use crate::core::rf::Rf;
use crate::core::slot::Slot;
use std::sync::atomic::{AtomicU64, Ordering};

pub type NodeRef = Rf<Node>;
pub type NodeId = u64;

static NODE_ID_GENERATOR: AtomicU64 = AtomicU64::new(1);

pub struct Node {
    pub id: NodeId,
    pub slots: Vec<Rf<Slot>>,
    pub providers: Vec<Rf<Provider>>,
    inner: Box<dyn NodeInner>,
}

pub trait NodeInner {
    fn new() -> Self
        where
            Self: Sized;
    fn get_slots(self: &Self) -> Vec<Rf<Slot>>;
    fn get_providers(self: &Self) -> Vec<Rf<Provider>>;
    fn run(self: &mut Self);
}

impl Node {
    pub fn new<T: 'static + NodeInner>() -> NodeRef {
        let inner = Box::new(T::new());
        let rf = Rf::new(Node {
            id: NODE_ID_GENERATOR.fetch_add(1, Ordering::Relaxed),
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
}
