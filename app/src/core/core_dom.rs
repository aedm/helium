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
}

impl RenderThread {
    fn start(&mut self) {
        loop {
            println!("R: wake");
            while let Ok(x) = self.receiver.try_recv() {
                match *x {
                    CoreMessage::Mutate(mut x) => {
                        println!("R: mutate");
                        x.run();
                    }
                    CoreMessage::Stop => {
                        println!("R: stop");
                        return;
                    }
                }
            }
            self.core_root.borrow_mut().run();
            println!("Core: {:?}", self.core_root.borrow().slots[0].borrow().get_single_provider().unwrap().borrow().provider_value);
            thread::sleep(Duration::from_millis(100));
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
        };

        let join_handle = thread::Builder::new()
            .name("render core".into())
            .spawn(move || {
                println!("Core thread start. {:?}", thread::current().id());
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
        dbg!();
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