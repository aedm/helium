use crate::core::node::{CoreNodeRef, CoreNode};
use crate::nodes::root_node::CoreRootNode;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver, Sender, channel};
use crate::core::core_mutation::CoreMutationSequence;

pub struct CoreDom {
    pub core_root: CoreNodeRef,
    join_handle: JoinHandle<()>,
    receiver: Receiver<Box<CoreMutationSequence>>,
    pub sender: Sender<Box<CoreMutationSequence>>,
}

impl CoreDom {
    pub fn new() -> CoreDom {
        let join_handle = thread::Builder::new()
            .name("render core".into())
            .spawn(move || {
                println!("Core thread start. {:?}", thread::current().id());
            })
            .unwrap();

        let (sender, receiver) = channel();

        CoreDom {
            core_root: CoreNode::new::<CoreRootNode>(),
            join_handle,
            sender,
            receiver,
        }
    }

    pub fn _stop(&mut self) {
        // (&self.join_handle).join().unwrap();
    }
}