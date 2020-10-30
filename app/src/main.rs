use crate::core::core_mutation::{
    CoreMutationSequence, SetNodeDependencyListCoreMutation, SetSlotConnectionsCoreMutation,
};
use crate::core::node::{CoreNode, CoreProviderIndex, CoreSlotIndex};
use crate::core::slot::{CoreSlotDefault};
use flow::topological_order::TopologicalOrder;
use crate::flow::dom::Dom;
use crate::flow::flow_node::{FlowNode, FlowSlotIndex};
use crate::flow::mutation::{FlowMutation, FlowMutationStepResult};
use crate::nodes::float_node::FloatNode;
use crate::nodes::sum_node::SumNode;
use crate::stillaxis::Stillaxis;
use std::any::TypeId;
use crate::flow::mutation_set_connections::SetSlotConnectionsFlowMutation;
use crate::flow::mutation_create_node::CreateNodeFlowMutation;
use std::borrow::Borrow;

mod core;
mod flow;
mod nodes;
mod stillaxis;

fn case_3() {
    println!("---- case 3 ----");
    let mut dom = Dom::new();

    let cf1 = CoreNode::new::<FloatNode>();
    let ff1 = FlowNode::from_core_node(&cf1);
    let m1 = Box::new(CreateNodeFlowMutation {
        new_node: ff1.clone(),
    });

    let cf2 = CoreNode::new::<FloatNode>();
    let ff2 = FlowNode::from_core_node(&cf2);
    let m2 = Box::new(CreateNodeFlowMutation {
        new_node: ff2.clone(),
    });

    let csum = CoreNode::new::<SumNode>();
    let fsum = FlowNode::from_core_node(&csum);
    let m3 = Box::new(CreateNodeFlowMutation {
        new_node: fsum.clone(),
    });

    let m4 = Box::new(SetSlotConnectionsFlowMutation {
        node_slot: FlowSlotIndex {
            node: fsum.clone(),
            slot_index: 0,
        },
        connections: vec![FlowSlotIndex {
            node: ff1.clone(),
            slot_index: 0,
        }],
    });

    let m5 = Box::new(SetSlotConnectionsFlowMutation {
        node_slot: FlowSlotIndex {
            node: fsum.clone(),
            slot_index: 1,
        },
        connections: vec![FlowSlotIndex {
            node: ff2.clone(),
            slot_index: 0,
        }],
    });

    let mut mutseq = FlowMutation::new(vec![m1, m2, m3, m4, m5]);

    let mut core_mutation = mutseq.run(&mut dom);
    core_mutation.run();

    csum.borrow_mut().run_deps();

    println!("{:?}", csum.borrow().providers[0].borrow().provider_value);
}

fn case_2() {
    println!("---- case 2 ----");
    let f1 = CoreNode::new::<FloatNode>();
    let f2 = CoreNode::new::<FloatNode>();
    let sum = CoreNode::new::<SumNode>();

    let conn_1 = Box::new(SetSlotConnectionsCoreMutation {
        slot: CoreSlotIndex { node: sum.clone(), slot_index: 0 },
        connection: vec![CoreProviderIndex { node: f1.clone(), provider_index: 0 }],
        swap_vector: Vec::with_capacity(1),
    });
    let conn_2 = Box::new(SetSlotConnectionsCoreMutation {
        slot: CoreSlotIndex { node: sum.clone(), slot_index: 1 },
        connection: vec![CoreProviderIndex { node: f2.clone(), provider_index: 0 }],
        swap_vector: Vec::with_capacity(1),
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

fn main() {
    case_3();
    case_2();
}
