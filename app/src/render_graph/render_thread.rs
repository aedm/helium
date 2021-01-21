use crate::render_graph::render_thread::MessageToRenderThread::Stop;
use crate::render_graph::mutation::MutationSequence;
use crate::render_graph::node::{Node, NodeProviderRef};
use crate::render_graph::node_ref::NodeRef;
use crate::render_graph::provider::ProviderValue;
use crate::nodes::root_node::CoreRootNode;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::{JoinHandle, ThreadId};
use std::time::Duration;
use strum_macros::IntoStaticStr;

#[derive(IntoStaticStr)]
pub enum MessageToRenderThread {
    Mutate(MutationSequence),
    Stop,
    GetProviderValue(ProviderValueRequest),
}

unsafe impl Send for MessageToRenderThread {}

pub struct RenderGraph {
    pub root_node: NodeRef,
    join_handle: Option<JoinHandle<()>>,
    pub sender_to_render_thread: Sender<Box<MessageToRenderThread>>,
    pub receiver_from_render_thread: Receiver<Box<MessageToRenderThread>>,
    node_id_generator: AtomicU64,
}

pub struct RenderThread {
    root_node: NodeRef,
    receiver_to_render_thread: Receiver<Box<MessageToRenderThread>>,
    sender_from_render_thread: Sender<Box<MessageToRenderThread>>,
    frame_count: u64,
}

pub struct ProviderValueRequest {
    pub provider: NodeProviderRef,
    pub response_value: Option<ProviderValue>,
}

impl RenderThread {
    fn start(&mut self) {
        println!("R: start");
        loop {
            while let Ok(mut x) = self.receiver_to_render_thread.try_recv() {
                match x.as_mut() {
                    MessageToRenderThread::Mutate(mutation) => {
                        mutation.run();
                    }
                    MessageToRenderThread::Stop => {
                        println!("R: stop. Total frame count: {}", self.frame_count);
                        return;
                    }
                    MessageToRenderThread::GetProviderValue(request) => {
                        RenderThread::run_node_deps(&mut self.root_node);
                        Self::handle_provider_value_request(request);
                    }
                }
                // Send message back to main thread
                let _ = self.sender_from_render_thread.send(x);
            }
            self.frame_count += 1;
            RenderThread::run_node_deps(&mut self.root_node);
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn handle_provider_value_request(request: &mut ProviderValueRequest) {
        let node = request.provider.node.borrow_mut();
        let provider = node.descriptor().providers[request.provider.provider_index].borrow();
        request.response_value = Some(provider.provider_value.clone());
    }

    pub fn run_node_deps(node_ref: &NodeRef) {
        let mut node = node_ref.borrow_mut();
        for dep in &node.descriptor().dependency_list {
            dep.borrow_mut().run();
        }
        node.run();
    }
}

impl RenderGraph {
    pub fn new() -> RenderGraph {
        let (sender_to_render_thread, receiver_to_render_thread) = channel();
        let (sender_from_render_thread, receiver_from_render_thread) = channel();
        let root_node = NodeRef::new(Box::new(CoreRootNode::new(0)));

        let mut render_thread = RenderThread {
            receiver_to_render_thread,
            sender_from_render_thread,
            root_node: root_node.clone(),
            frame_count: 0,
        };

        let join_handle = thread::Builder::new()
            .name("render render_graph".into())
            .spawn(move || {
                render_thread.start();
            })
            .unwrap();

        RenderGraph {
            root_node,
            join_handle: Some(join_handle),
            sender_to_render_thread,
            receiver_from_render_thread,
            node_id_generator: AtomicU64::new(1),
        }
    }

    pub fn new_node<T: 'static + Node>(&self) -> NodeRef {
        let id = self.node_id_generator.fetch_add(1, Ordering::Relaxed);
        let core_node = NodeRef::new(Box::new(T::new(id)));
        core_node
            .borrow_mut()
            .descriptor_mut()
            .seal(self.get_render_thread_id(), &core_node);
        core_node
    }

    fn get_render_thread_id(&self) -> ThreadId {
        match &self.join_handle {
            Some(handle) => handle.thread().id(),
            None => panic!(),
        }
    }

    pub fn stop(&mut self) {
        println!("Stopping render thread...");
        let _ = self.sender_to_render_thread.send(Box::new(Stop));
        self.join_handle.take().map(JoinHandle::join);
        println!("Render thread stopped.");
    }
}

impl Drop for RenderGraph {
    fn drop(&mut self) {
        dbg!("CoreDom.drop");
        self.stop();
    }
}
