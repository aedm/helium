use crate::flow::provider::{FloatProvider, ProviderValue};
use crate::flow::node::{Node};
use crate::flow::rf::Rf;
use crate::flow::slot::{connect_slot, SlotDefault};
use crate::nodes::sum_node::SumNode;
use crate::nodes::float_node::FloatNode;
use std::borrow::BorrowMut;

mod flow;
mod nodes;

fn main() {
    let f1 = Node::new::<FloatNode>();
    let f2 = Node::new::<FloatNode>();
    let sum = Node::new::<SumNode>();

    let mut f1p = f1.borrow_mut();
    let mut f2p = f2.borrow_mut();
    let mut a1s = sum.borrow_mut();
    connect_slot(&a1s.slots[0], &f1p.providers[0]);
    connect_slot(&a1s.slots[1], &f2p.providers[0]);

    f1p.slots[0].borrow_mut().set_default(SlotDefault::Float32(5.0));

    f1p.run();
    f2p.run();
    a1s.run();

    println!("Result: {:?}", a1s.providers[0].borrow().value);
}