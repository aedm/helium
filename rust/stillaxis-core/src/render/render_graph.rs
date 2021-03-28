use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::{JoinHandle, ThreadId};

use strum_macros::IntoStaticStr;

use crate::mutation::MutationSequence;
use crate::node::{Node, ProviderRef};
use crate::node_ref::NodeRef;
use crate::nodes::root_node::RootNode;
use crate::provider::ProviderValue;
use crate::render::render_thread::RenderThread;
use crate::render::render_graph::Message::Stop;

#[derive(IntoStaticStr)]
pub enum Message {
    Mutate(MutationSequence),
    Stop,
    GetProviderValue(ProviderValueRequest),
}

unsafe impl Send for Message {}

pub struct RenderGraph {
    pub root_node: NodeRef,
    pub sender_to_render_thread: Sender<Box<Message>>,
    pub receiver_from_render_thread: Receiver<Box<Message>>,
    render_thread_join_handle: Option<JoinHandle<()>>,
    node_id_generator: AtomicU64,
}

pub struct ProviderValueRequest {
    pub provider: ProviderRef,
    pub response_value: Option<ProviderValue>,
}

impl RenderGraph {
    pub fn new() -> RenderGraph {
        let (sender_to_render_thread, receiver_to_render_thread) = channel();
        let (sender_from_render_thread, receiver_from_render_thread) = channel();
        let root_node = NodeRef::new(Box::new(RootNode::new(0)));

        let mut render_thread = RenderThread {
            receiver_to_render_thread,
            sender_from_render_thread,
            root_node: root_node.clone(),
            frame_count: 0,
        };

        let join_handle = thread::Builder::new()
            .name("render core".into())
            .spawn(move || {
                render_thread.start();
            })
            .unwrap();

        RenderGraph {
            root_node,
            render_thread_join_handle: Some(join_handle),
            sender_to_render_thread,
            receiver_from_render_thread,
            node_id_generator: AtomicU64::new(1),
        }
    }

    pub fn new_node<T: 'static + Node>(&self) -> NodeRef {
        let id = self.node_id_generator.fetch_add(1, Ordering::Relaxed);
        let node = NodeRef::new(Box::new(T::new(id)));
        node
            .borrow_mut()
            .descriptor_mut()
            .seal(self.get_render_thread_id(), &node);
        node
    }

    fn get_render_thread_id(&self) -> ThreadId {
        match &self.render_thread_join_handle {
            Some(handle) => handle.thread().id(),
            None => panic!(),
        }
    }

    pub fn stop(&mut self) {
        println!("Stopping render thread...");
        let _ = self.sender_to_render_thread.send(Box::new(Stop));
        self.render_thread_join_handle.take().map(JoinHandle::join);
        println!("Render thread stopped.");
    }
}

impl Drop for RenderGraph {
    fn drop(&mut self) {
        dbg!("CoreDom.drop");
        self.stop();
    }
}
