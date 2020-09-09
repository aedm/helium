use crate::provider::FloatProvider;
use crate::node::{new_aref, Adder, FloatNode, Node};
use crate::slot::connect_slot;
use std::cell::RefCell;

mod node;
mod provider;
mod values;
mod slot;

fn main() {
    let f1 = new_aref(Node::new(Box::new(FloatNode::new())));
    let f2 = new_aref(Node::new(Box::new(FloatNode::new())));
    let adder = new_aref(Node::new(Box::new(Adder::new())));

    let mut f1p = &mut *f1.borrow_mut();
    let mut f2p = &mut *f2.borrow_mut();
    let mut a1s = &mut *adder.borrow_mut();
    connect_slot(&mut a1s.slots[0], &mut f1p.providers[0]);
    connect_slot(&mut a1s.slots[0], &mut f2p.providers[0]);

    a1s.inner.run();

    println!("Hi");

}