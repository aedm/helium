use crate::node_ref::NodeRef;
use crate::render::render_graph::{Message, ProviderValueRequest};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub struct RenderThread {
    pub root_node: NodeRef,
    pub receiver_to_render_thread: Receiver<Box<Message>>,
    pub sender_from_render_thread: Sender<Box<Message>>,
    pub frame_count: u64,
}

impl RenderThread {
    pub fn start(&mut self) {
        println!("R: start");
        loop {
            while let Ok(mut x) = self.receiver_to_render_thread.try_recv() {
                match x.as_mut() {
                    Message::Mutate(mutation) => {
                        mutation.run();
                    }
                    Message::Stop => {
                        println!("R: stop. Total frame count: {}", self.frame_count);
                        return;
                    }
                    Message::GetProviderValue(request) => {
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
