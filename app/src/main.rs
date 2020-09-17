use crate::flow::node::Node;
use crate::flow::slot::{connect_slot, SlotDefault};
use crate::flow::topological_order::TopologicalOrder;
use crate::nodes::float_node::FloatNode;
use crate::nodes::sum_node::SumNode;

mod flow;
mod nodes;

fn main() {
    let f1 = Node::new::<FloatNode>();
    let f2 = Node::new::<FloatNode>();
    let sum = Node::new::<SumNode>();

    connect_slot(&sum.borrow_mut().slots[0], &f1.borrow_mut().providers[0]);
    connect_slot(&sum.borrow_mut().slots[1], &f2.borrow_mut().providers[0]);
    f1.borrow_mut().slots[0]
        .borrow_mut()
        .set_default(SlotDefault::Float32(5.0));

    let nodes = TopologicalOrder::generate(&sum);
    for node in &nodes {
        node.borrow_mut().run();
    }

    println!("Result: {:?}", sum.borrow().providers[0].borrow().value);
}
