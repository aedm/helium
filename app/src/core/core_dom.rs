use crate::core::node::{CoreNodeRef, CoreNode};
use crate::nodes::root_node::CoreRootNode;
use std::thread;
use std::thread::{JoinHandle, Result};
use std::sync::mpsc::{Receiver, Sender, channel};
use crate::core::core_mutation::CoreMutationSequence;
use std::time::Duration;
use crate::core::core_dom::CoreMessage::Stop;
use std::borrow::BorrowMut;

pub enum CoreMessage {
    Mutate(CoreMutationSequence),
    Stop,
}

unsafe impl Send for CoreMessage {}

pub struct CoreDom {
    pub core_root: CoreNodeRef,
    join_handle: Option<JoinHandle<()>>,
    pub sender: Sender<Box<CoreMessage>>,
}

struct RenderThread {
    core_root: CoreNodeRef,
    receiver: Receiver<Box<CoreMessage>>,
    frame_count: u64,
}

impl RenderThread {
    fn start(&mut self) {
        println!("R: start");
        let mut should_print_state = true;
        loop {
            while let Ok(x) = self.receiver.try_recv() {
                match *x {
                    CoreMessage::Mutate(mut x) => {
                        println!("R: mutate");
                        x.run();
                        should_print_state = true;
                    }
                    CoreMessage::Stop => {
                        println!("R: stop. Total frame count: {}", self.frame_count);
                        return;
                    }
                }
            }
            self.frame_count += 1;
            self.core_root.borrow_mut().run_deps();
            if should_print_state {
                self.print_state();
                should_print_state = false;
            }
        }
    }

    fn print_state(&self) {
        let mut core_root = self.core_root.borrow();
        let connections = &core_root.slots[0].borrow().connection;
        println!("R: Connection count: {}", connections.len());
        for provider in connections {
            println!("R:   provider value: {:?}", provider.borrow().provider_value);
        }
    }
}

impl CoreDom {
    pub fn new() -> CoreDom {
        let (sender, receiver) = channel();
        let core_root = CoreNode::new::<CoreRootNode>();

        let mut render_thread = RenderThread {
            receiver,
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
            sender,
        }
    }

    pub fn stop(&mut self) {
        println!("Stopping render thread...");
        let _ = self.sender.send(Box::new(Stop));
        self.join_handle.take().map(JoinHandle::join);
        println!("Render thread stopped.");
    }
}

impl Drop for CoreDom {
    fn drop(&mut self) {
        dbg!();
        self.stop();
    }
}