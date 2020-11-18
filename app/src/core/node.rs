use crate::core::acell::ACell;
use crate::core::node_ref::CoreNodeRef;
use crate::core::provider::CoreProvider;
use crate::core::slot::CoreSlot;
use core::fmt;
use std::fmt::{Debug, Formatter};
use std::thread;
use std::thread::ThreadId;

pub type NodeId = u64;

pub struct CoreNodeInner {
    pub id: NodeId,
    pub name: String,
    pub slots: Vec<ACell<CoreSlot>>,
    pub providers: Vec<ACell<CoreProvider>>,
    pub dependency_list: Vec<CoreNodeRef>,
    render_thread_id: Option<ThreadId>,
    type_name: &'static str,
}

pub struct CoreProviderIndex {
    pub node: CoreNodeRef,
    pub provider_index: usize,
}

pub struct CoreSlotIndex {
    pub node: CoreNodeRef,
    pub slot_index: usize,
}

pub trait CoreNode: Debug {
    fn new(id: NodeId) -> Self
    where
        Self: std::marker::Sized;

    fn get_inner(&self) -> &CoreNodeInner;
    fn get_inner_mut(&mut self) -> &mut CoreNodeInner;

    fn run(&mut self);

    fn get_type_name(&self) -> &'static str {
        self.get_inner().type_name
    }

    fn get_slots(&self) -> &Vec<ACell<CoreSlot>> {
        &self.get_inner().slots
    }

    fn get_providers(&self) -> &Vec<ACell<CoreProvider>> {
        &self.get_inner().providers
    }

    fn get_id(&self) -> NodeId {
        self.get_inner().id
    }

    fn get_name(&self) -> &str {
        &self.get_inner().name
    }
}

impl CoreNodeInner {
    pub fn new(
        id: NodeId,
        type_name: &'static str,
        slots: Vec<ACell<CoreSlot>>,
        providers: Vec<ACell<CoreProvider>>,
    ) -> CoreNodeInner {
        CoreNodeInner {
            id,
            name: format!("{}-{}", type_name, id),
            dependency_list: vec![],
            slots,
            providers,
            render_thread_id: None,
            type_name,
        }
    }

    // fn run(&mut self) {
    //     debug_assert!(self.check_render_thread(true));
    //     self.inner.run();
    // }

    // pub fn run_deps(&mut self) {
    //     for dep in &self.dependency_list {
    //         dep.borrow_mut().run();
    //     }
    //     self.run();
    // }

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

impl Debug for CoreNodeInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("'{}'({})", self.name, self.id))
    }
}

// impl<T: CoreNode> Debug for T {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         self.get_inner().fmt(f)
//     }
// }

impl Drop for CoreNodeInner {
    fn drop(&mut self) {
        // Core node should never be deallocated on the render thread
        debug_assert!(self.check_render_thread(false));
        println!("Core node drop: {:?}", self);
    }
}
