use crate::provider::FloatProvider;
use crate::node::{Adder, FloatNode, Node};
use crate::slot::connect_slot;
use std::cell::RefCell;
use crate::rf::Rf;

mod node;
mod provider;
mod slot;
mod rf;

fn main() {
    let f1 = Rf::new(Node::new::<FloatNode>());
    let f2 = Rf::new(Node::new::<FloatNode>());
    let adder = Rf::new(Node::new::<Adder>());
    // let f1 = Rf::new(Node::new(Box::new(FloatNode::new())));
    // let f2 = Rf::new(Node::new(Box::new(FloatNode::new())));
    // let adder = Adder::make_node();

    let mut f1p = f1.borrow_mut();
    let mut f2p = f2.borrow_mut();
    let mut a1s = adder.borrow_mut();
    connect_slot(&a1s.slots[0], &f1p.providers[0]);
    connect_slot(&a1s.slots[1], &f2p.providers[0]);

    f1p.run();
    f2p.run();
    a1s.run();

    println!("Hi");
}