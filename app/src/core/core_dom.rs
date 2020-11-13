use crate::core::core_dom::CoreMessage::Stop;
use crate::core::core_mutation::CoreMutationSequence;
use crate::core::node::{CoreNode, CoreNodeRef, CoreProviderIndex, NodeInner};
use crate::core::provider::CoreProviderValue;
use crate::nodes::root_node::CoreRootNode;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::{JoinHandle, ThreadId};
use std::time::Duration;
use strum_macros::IntoStaticStr;

#[derive(IntoStaticStr)]
pub enum CoreMessage {
    Mutate(CoreMutationSequence),
    Stop,
    GetProviderValue(ProviderValueRequest),
}

unsafe impl Send for CoreMessage {}

pub struct CoreDom {
    pub core_root: CoreNodeRef,
    join_handle: Option<JoinHandle<()>>,
    pub sender_to_render_thread: Sender<Box<CoreMessage>>,
    pub receiver_from_render_thread: Receiver<Box<CoreMessage>>,
}

pub struct RenderThread {
    core_root: CoreNodeRef,
    receiver_to_render_thread: Receiver<Box<CoreMessage>>,
    sender_from_render_thread: Sender<Box<CoreMessage>>,
    frame_count: u64,
}

pub struct ProviderValueRequest {
    pub provider: CoreProviderIndex,
    pub response_value: Option<CoreProviderValue>,
}

impl RenderThread {
    fn start(&mut self) {
        println!("R: start");
        loop {
            while let Ok(mut x) = self.receiver_to_render_thread.try_recv() {
                match x.as_mut() {
                    CoreMessage::Mutate(mutation) => {
                        mutation.run();
                    }
                    CoreMessage::Stop => {
                        println!("R: stop. Total frame count: {}", self.frame_count);
                        return;
                    }
                    CoreMessage::GetProviderValue(request) => {
                        self.core_root.borrow_mut().run_deps();
                        Self::handle_provider_value_request(request);
                    }
                }
                // Send message back to main thread
                let _ = self.sender_from_render_thread.send(x);
            }
            self.frame_count += 1;
            self.core_root.borrow_mut().run_deps();
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn handle_provider_value_request(request: &mut ProviderValueRequest) {
        let node = request.provider.node.borrow_mut();
        let provider = node.providers[request.provider.provider_index].borrow();
        request.response_value = Some(provider.provider_value);
    }
}

impl CoreDom {
    pub fn new() -> CoreDom {
        let (sender_to_render_thread, receiver_to_render_thread) = channel();
        let (sender_from_render_thread, receiver_from_render_thread) = channel();
        let core_root = CoreNode::new::<CoreRootNode>();

        let mut render_thread = RenderThread {
            receiver_to_render_thread,
            sender_from_render_thread,
            core_root: core_root.clone(),
            frame_count: 0,
        };

        let join_handle = thread::Builder::new()
            .name("render core".into())
            .spawn(move || {
                render_thread.start();
            })
            .unwrap();

        CoreDom {
            core_root,
            join_handle: Some(join_handle),
            sender_to_render_thread,
            receiver_from_render_thread,
        }
    }

    pub fn new_node<T: 'static + NodeInner>(&self) -> CoreNodeRef {
        let core_node = CoreNode::new::<T>();
        core_node.borrow_mut().seal(self.get_render_thread_id());
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

impl Drop for CoreDom {
    fn drop(&mut self) {
        dbg!("CoreDom.drop");
        self.stop();
    }
}
