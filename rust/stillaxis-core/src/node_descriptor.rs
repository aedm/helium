use crate::node_ref::NodeRef;
use crate::provider::Provider;
use crate::providers::node_provider::NodeProvider;
use crate::rcell::RCell;
use crate::slot::Slot;
use std::thread::ThreadId;
use std::{fmt, thread};

pub type NodeId = u64;

pub struct NodeDescriptor {
    pub id: NodeId,
    pub name: String,
    pub slots: Vec<RCell<Slot>>,
    pub providers: Vec<RCell<Provider>>,
    pub dependency_list: Vec<NodeRef>,
    render_thread_id: Option<ThreadId>,
    type_name: &'static str,
    node_provider: NodeProvider,
}

impl NodeDescriptor {
    pub fn new(
        id: NodeId,
        type_name: &'static str,
        slots: Vec<RCell<Slot>>,
        mut providers: Vec<RCell<Provider>>,
    ) -> NodeDescriptor {
        // Always add NodeProvider as #0 provider
        let node_provider = NodeProvider::new("node");
        providers.insert(0, node_provider.provider.clone());
        NodeDescriptor {
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

    pub fn seal(&mut self, render_thread_id: ThreadId, owner: &NodeRef) {
        self.node_provider.set(owner);
        self.render_thread_id = Some(render_thread_id);
    }

    fn check_render_thread(&self, is_render_thread: bool) -> bool {
        match self.render_thread_id {
            Some(thread_id) => (thread_id == thread::current().id()) == is_render_thread,
            None => true,
        }
    }
}

impl fmt::Debug for NodeDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!(
            "'{}'(type='{}',id={})",
            self.name, self.type_name, self.id
        ))
    }
}

impl Drop for NodeDescriptor {
    fn drop(&mut self) {
        // Node should never be deallocated on the render thread
        debug_assert!(self.check_render_thread(false));
        println!("Core node drop: {:?}", self);
    }
}
