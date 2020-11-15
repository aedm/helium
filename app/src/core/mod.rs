pub mod acell;
pub mod core_dom;
pub mod core_mutation;
pub mod node;
pub mod provider;
pub mod rcell;
pub mod slot;

#[cfg(test)]
mod module_tests {
    use crate::core::core_mutation::{
        CoreMutation, CoreMutationSequence, SetNodeDependencyListParams, SetSlotConnectionsParams,
    };
    use crate::core::node::{CoreNode, CoreProviderIndex, CoreSlotIndex};
    use crate::core::provider::CoreProviderValue;
    use crate::nodes::float_node::FloatNode;
    use crate::nodes::sum_node::SumNode;

    #[test]
    fn generates_simple_sum_graph() {
        let f1 = CoreNode::new::<FloatNode>(1);
        let f2 = CoreNode::new::<FloatNode>(2);
        let sum = CoreNode::new::<SumNode>(3);

        let conn_1 = CoreMutation::SetSlotConnections(SetSlotConnectionsParams {
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
        let conn_2 = CoreMutation::SetSlotConnections(SetSlotConnectionsParams {
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
        let dep = CoreMutation::SetNodeDependencyList(SetNodeDependencyListParams {
            node: sum.clone(),
            dependency_list: vec![f1.clone(), f2.clone()],
        });
        let mut seq = CoreMutationSequence {
            steps: vec![conn_1, conn_2, dep],
        };
        seq.run();

        sum.borrow_mut().run_deps();
        assert_eq!(
            sum.borrow().providers[0].borrow().provider_value,
            CoreProviderValue::Float32(0.0)
        );

        // TODO: test non-zero values
    }
}
