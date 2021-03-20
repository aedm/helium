pub mod core_dom;
pub mod core_mutation;
pub mod core_node_descriptor;
pub mod node;
pub mod node_ref;
pub mod nodes;
pub mod provider;
pub mod providers;
pub mod rcell;
pub mod slot;
pub mod slots;

#[cfg(test)]
mod module_tests {
    use crate::core_dom::{CoreDom, RenderThread};
    use crate::core_mutation::{
        CoreMutation, CoreMutationSequence, SetNodeDependencyListParams, SetSlotConnectionsParams,
    };
    use crate::node::{CoreProviderIndex, CoreSlotIndex};
    use crate::nodes::float_node::FloatNode;
    use crate::nodes::sum_node::SumNode;
    use crate::provider::CoreProviderValue;

    #[test]
    fn generates_simple_sum_graph() {
        let dom = CoreDom::new();
        let f1 = dom.new_node::<FloatNode>();
        let f2 = dom.new_node::<FloatNode>();
        let sum = dom.new_node::<SumNode>();

        let conn_1 = CoreMutation::SetSlotConnections(SetSlotConnectionsParams {
            slot: CoreSlotIndex {
                node: sum.clone(),
                slot_index: 0,
            },
            connection: vec![CoreProviderIndex {
                node: f1.clone(),
                provider_index: 1,
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
                provider_index: 1,
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

        RenderThread::run_node_deps(&sum);
        assert_eq!(
            sum.borrow().descriptor().providers[1]
                .borrow()
                .provider_value,
            CoreProviderValue::Float32(0.0)
        );

        // TODO: test non-zero values
    }
}
