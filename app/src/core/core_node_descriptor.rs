use crate::core::node_ref::CoreNodeRef;
use crate::core::provider::CoreProvider;
use crate::core::rcell::RCell;
use crate::core::slot::CoreSlot;
use std::thread::ThreadId;
use std::{fmt, thread};
use crate::providers::node_provider::NodeCoreProvider;

pub type NodeId = u64;

pub struct CoreNodeDescriptor {
    pub id: NodeId,
    pub name: String,
    pub slots: Vec<RCell<CoreSlot>>,
    pub providers: Vec<RCell<CoreProvider>>,
    pub dependency_list: Vec<CoreNodeRef>,
    render_thread_id: Option<ThreadId>,
    type_name: &'static str,
    node_provider: NodeCoreProvider,
}

impl CoreNodeDescriptor {
    pub fn new(
        id: NodeId,
        type_name: &'static str,
        slots: Vec<RCell<CoreSlot>>,
        mut providers: Vec<RCell<CoreProvider>>,
    ) -> CoreNodeDescriptor {
        // Always add NodeProvider as #0 provider
        let node_provider = NodeCoreProvider::new("node");
        providers.insert(0, node_provider.provider.clone());
        CoreNodeDescriptor {
            id,
            node_provider,
            name: format!("{}-{}", type_name, id),
            dependency_list: vec![],
            slots,
            providers,
            render_thread_id: None,
            type_name,
        }
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

impl fmt::Debug for CoreNodeDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!(
            "'{}'(type='{}',id={})",
            self.name, self.type_name, self.id
        ))
    }
}

impl Drop for CoreNodeDescriptor {
    fn drop(&mut self) {
        // Core node should never be deallocated on the render thread
        debug_assert!(self.check_render_thread(false));
        println!("Core node drop: {:?}", self);
    }
}
