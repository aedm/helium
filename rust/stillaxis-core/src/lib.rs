pub mod mutation;
pub mod node;
pub mod node_descriptor;
pub mod node_ref;
pub mod nodes;
pub mod provider;
pub mod providers;
pub mod rcell;
pub mod render;
pub mod slot;
pub mod slots;

#[cfg(test)]
mod module_tests {
    use crate::mutation::{
        Mutation, MutationSequence, SetNodeDependencyListParams, SetSlotConnectionsParams,
    };
    use crate::node::{ProviderRef, SlotRef};
    use crate::nodes::float_node::FloatNode;
    use crate::nodes::sum_node::SumNode;
    use crate::provider::ProviderValue;
    use crate::render::render_graph::RenderGraph;
    use crate::render::render_thread::RenderThread;

    #[test]
    fn generates_simple_sum_graph() {
        let dom = RenderGraph::new();
        let f1 = dom.new_node::<FloatNode>();
        let f2 = dom.new_node::<FloatNode>();
        let sum = dom.new_node::<SumNode>();

        let conn_1 = Mutation::SetSlotConnections(SetSlotConnectionsParams {
            slot: SlotRef {
                node: sum.clone(),
                slot_index: 0,
            },
            connection: vec![ProviderRef {
                node: f1.clone(),
                provider_index: 1,
            }],
            swap_vector: Vec::with_capacity(1),
        });
        let conn_2 = Mutation::SetSlotConnections(SetSlotConnectionsParams {
            slot: SlotRef {
                node: sum.clone(),
                slot_index: 1,
            },
            connection: vec![ProviderRef {
                node: f2.clone(),
                provider_index: 1,
            }],
            swap_vector: Vec::with_capacity(1),
        });
        let dep = Mutation::SetNodeDependencyList(SetNodeDependencyListParams {
            node: sum.clone(),
            dependency_list: vec![f1.clone(), f2.clone()],
        });
        let mut seq = MutationSequence {
            steps: vec![conn_1, conn_2, dep],
        };
        seq.run();

        RenderThread::run_node_deps(&sum);
        assert_eq!(
            sum.borrow().descriptor().providers[1]
                .borrow()
                .provider_value,
            ProviderValue::Float32(0.0)
        );

        // TODO: test non-zero values
    }
}
