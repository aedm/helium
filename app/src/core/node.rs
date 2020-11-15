use crate::core::acell::ACell;
use crate::core::provider::CoreProvider;
use crate::core::slot::CoreSlot;
use std::any::{Any, TypeId};
use std::thread;
use std::thread::ThreadId;

pub type CoreNodeRef = ACell<CoreNode>;
pub type NodeId = u64;

pub struct CoreNode {
    // pub type_name: &'static str,
    // pub name: String,
    pub slots: Vec<ACell<CoreSlot>>,
    pub providers: Vec<ACell<CoreProvider>>,
    pub inner: Box<dyn NodeInner>,
    pub dependency_list: Vec<CoreNodeRef>,
    render_thread_id: Option<ThreadId>,
}

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
            render_thread_id: None,
        });
        rf
    }

    pub fn run(&mut self) {
        debug_assert!(self.check_render_thread(true));
        self.inner.run();
    }

    pub fn run_deps(&mut self) {
        for dep in &self.dependency_list {
            dep.borrow_mut().run();
        }
        self.run();
    }

    pub fn seal(&mut self, render_thread_id: ThreadId) {
        self.render_thread_id = Some(render_thread_id);
    }

    fn check_render_thread(&self, is_render_thread: bool) -> bool {
        match self.render_thread_id {
            Some(thread_id) => (thread_id == thread::current().id()) == is_render_thread,
            None => true,
        }
    }
}

impl Drop for CoreNode {
    fn drop(&mut self) {
        // Core node should never be deallocated on the render thread
        debug_assert!(self.check_render_thread(false));
        println!("Core node drop");
    }
}
