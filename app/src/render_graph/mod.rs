pub mod render_thread;
pub mod graph_mutation;
pub mod node_descriptor;
pub mod node;
pub mod node_ref;
pub mod provider;
pub mod rcell;
pub mod slot;

#[cfg(test)]
mod module_tests {
    use crate::render_graph::render_thread::{RenderGraph, RenderThread};
    use crate::render_graph::graph_mutation::{
        GraphMutation, GraphMutationSequence, SetNodeDependencyListParams, SetSlotConnectionsParams,
    };
    use crate::render_graph::node::{NodeProviderRef, NodeSlotRef};
    use crate::render_graph::provider::ProviderValue;
    use crate::nodes::float_node::FloatNode;
    use crate::nodes::sum_node::SumNode;

    #[test]
    fn generates_simple_sum_graph() {
        let dom = RenderGraph::new();
        let f1 = dom.new_node::<FloatNode>();
        let f2 = dom.new_node::<FloatNode>();
        let sum = dom.new_node::<SumNode>();

        let conn_1 = GraphMutation::SetSlotConnections(SetSlotConnectionsParams {
            slot: NodeSlotRef {
                node: sum.clone(),
                slot_index: 0,
            },
            connection: vec![NodeProviderRef {
                node: f1.clone(),
                provider_index: 1,
            }],
            swap_vector: Vec::with_capacity(1),
        });
        let conn_2 = GraphMutation::SetSlotConnections(SetSlotConnectionsParams {
            slot: NodeSlotRef {
                node: sum.clone(),
                slot_index: 1,
            },
            connection: vec![NodeProviderRef {
                node: f2.clone(),
                provider_index: 1,
            }],
            swap_vector: Vec::with_capacity(1),
        });
        let dep = GraphMutation::SetNodeDependencyList(SetNodeDependencyListParams {
            node: sum.clone(),
            dependency_list: vec![f1.clone(), f2.clone()],
        });
        let mut seq = GraphMutationSequence {
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
