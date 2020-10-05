use crate::core::core_mutation::{
    CoreMutationSequence, SetNodeDependencyListCoreMutation, SetSlotConnectionsCoreMutation,
};
use crate::core::node::CoreNode;
use crate::core::slot::{connect_slot, CoreSlotConnection, CoreSlotDefault};
use crate::core::topological_order::TopologicalOrder;
use crate::nodes::float_node::FloatNode;
use crate::nodes::sum_node::SumNode;
use std::any::TypeId;
use crate::stillaxis::Stillaxis;
use crate::flow::flow_node::FlowNode;
use crate::flow::mutation::CreateNodeFlowMutation;

mod core;
mod flow;
mod nodes;
mod stillaxis;

fn case_3() {
    println!("---- case 3 ----");
    let mut sa = Stillaxis::new();

    let cf1 = CoreNode::new::<FloatNode>();
    let ff1 = FlowNode::from_core_node(&cf1);
    let m1 = CreateNodeFlowMutation { new_node: ff1.clone() };

    let cf2 = CoreNode::new::<FloatNode>();
    let ff2 = FlowNode::from_core_node(&cf1);
    let m1 = CreateNodeFlowMutation { new_node: ff2.clone() };


}

fn case_2() {
    println!("---- case 2 ----");
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
        dependency_list: vec![f1.clone(), f2.clone()],
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

    println!("sum type: {:?}", sum.borrow_mut().inner_type_id());
    println!("f1 type: {:?}", f1.borrow_mut().inner_type_id());
    println!("f2 type: {:?}", f2.borrow_mut().inner_type_id());
    println!("floatnode type: {:?}", TypeId::of::<FloatNode>());
    println!("sumnode type: {:?}", TypeId::of::<SumNode>());

    {
        let rf = sum.borrow();
        let sumany = rf.inner.as_any().downcast_ref::<SumNode>();
        println!("{}", matches!(sumany, Some(_)));
        let s = sumany.unwrap();
        println!("{}", &s.a.get());
    }
}

fn case_1() {
    println!("---- case 1 ----");
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
    case_3();
    case_1();
    case_2();
}
