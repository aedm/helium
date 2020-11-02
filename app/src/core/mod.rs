pub mod acell;
pub mod core_mutation;
pub mod node;
pub mod provider;
pub mod rcell;
pub mod slot;

#[cfg(test)]
mod module_tests {
    use crate::core::node::{CoreNode, CoreSlotIndex, CoreProviderIndex};
    use crate::nodes::float_node::FloatNode;
    use crate::nodes::sum_node::SumNode;
    use crate::core::core_mutation::{SetSlotConnectionsCoreMutation, SetNodeDependencyListCoreMutation, CoreMutationSequence};
    use std::any::TypeId;

    #[test]
    fn generates_simple_sum_graph() {
        let f1 = CoreNode::new::<FloatNode>();
        let f2 = CoreNode::new::<FloatNode>();
        let sum = CoreNode::new::<SumNode>();

        let conn_1 = Box::new(SetSlotConnectionsCoreMutation {
            slot: CoreSlotIndex {
                node: sum.clone(),
                slot_index: 0,
            },
            connection: vec![CoreProviderIndex {
                node: f1.clone(),
                provider_index: 0,
            }],
            swap_vector: Vec::with_capacity(1),
        });
        let conn_2 = Box::new(SetSlotConnectionsCoreMutation {
            slot: CoreSlotIndex {
                node: sum.clone(),
                slot_index: 1,
            },
            connection: vec![CoreProviderIndex {
                node: f2.clone(),
                provider_index: 0,
            }],
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
}