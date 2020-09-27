use crate::core::node::CoreNode;
use crate::core::slot::{connect_slot, CoreSlotDefault, CoreSlotConnection};
use crate::core::topological_order::TopologicalOrder;
use crate::nodes::float_node::FloatNode;
use crate::nodes::sum_node::SumNode;
use crate::core::core_mutation::{SetSlotConnectionsCoreMutation, SetNodeDependencyListCoreMutation, CoreMutationSequence};

mod core;
mod flow;
mod nodes;

fn case_2() {
    let f1 = CoreNode::new::<FloatNode>();
    let f2 = CoreNode::new::<FloatNode>();
    let sum = CoreNode::new::<SumNode>();

    let conn_1 = Box::new(SetSlotConnectionsCoreMutation {
        slot: sum.borrow().slots[0].clone(),
        connection: CoreSlotConnection::Single(f1.borrow().providers[0].clone()),
    });
    let conn_2 = Box::new(SetSlotConnectionsCoreMutation {
        slot: sum.borrow().slots[1].clone(),
        connection: CoreSlotConnection::Single(f2.borrow().providers[0].clone()),
    });
    let dep = Box::new(SetNodeDependencyListCoreMutation {
        node: sum.clone(),
        dependency_list: vec![f1.clone(), f2.clone()]
    });
    let mut seq = CoreMutationSequence {
        steps: vec![conn_1, conn_2, dep],
    };
    seq.run();

    sum.borrow_mut().run_deps();
    println!(
        "Result: {:?}",
        sum.borrow().providers[0].borrow().provider_value
    );
}

fn case_1() {
    let f1 = CoreNode::new::<FloatNode>();
    let f2 = CoreNode::new::<FloatNode>();
    let sum = CoreNode::new::<SumNode>();

    connect_slot(&sum.borrow_mut().slots[0], &f1.borrow_mut().providers[0]);
    connect_slot(&sum.borrow_mut().slots[1], &f2.borrow_mut().providers[0]);
    f1.borrow_mut().slots[0]
        .borrow_mut()
        .set_default(CoreSlotDefault::Float32(5.0));

    let nodes = TopologicalOrder::generate(&sum);
    for node in &nodes {
        node.borrow_mut().run();
    }

    println!(
        "Result: {:?}",
        sum.borrow().providers[0].borrow().provider_value
    );
}

fn main() {
    case_1();
    case_2();
}
