use crate::flow::flow_node::FlowNodeRef;
use std::collections::HashSet;

pub struct TopologicalOrder {
    visited: HashSet<FlowNodeRef>,
    order: Vec<FlowNodeRef>,
}

impl TopologicalOrder {
    pub fn generate(node_ref: &FlowNodeRef) -> Vec<FlowNodeRef> {
        let mut order = TopologicalOrder {
            visited: HashSet::new(),
            order: Vec::new(),
        };
        order.visit(node_ref);
        order.order
    }

    fn visit(&mut self, node_ref: &FlowNodeRef) {
        if !self.visited.insert(node_ref.clone()) {
            return;
        }
        for slot in &node_ref.borrow().slots {
            for provider_ref in &slot.connections {
                self.visit(&provider_ref.node);
            }
        }
        self.order.push(node_ref.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::flow::flow_node::{FlowNode, FlowNodeRef, FlowProviderIndex};
    use crate::flow::topological_order::TopologicalOrder;
    use stillaxis_core::node_ref::CoreNodeRef;
    use stillaxis_core::nodes::float_node::FloatNode;
    use stillaxis_core::nodes::sum_node::SumNode;
    use stillaxis_core::node::CoreNode;

    fn connect(
        slot_node: &FlowNodeRef,
        slot_index: usize,
        provider_node: &FlowNodeRef,
        provider_index: usize,
    ) {
        slot_node.borrow_mut().slots[slot_index].connections = vec![FlowProviderIndex {
            node: provider_node.clone(),
            provider_index,
        }];
    }

    #[test]
    fn generates_correct_topological_order() {
        let float1 = CoreNodeRef::new(Box::new(FloatNode::new(1)));
        let float2 = CoreNodeRef::new(Box::new(FloatNode::new(2)));
        let sum1 = CoreNodeRef::new(Box::new(SumNode::new(3)));
        let sum2 = CoreNodeRef::new(Box::new(SumNode::new(4)));

        let core_nodes = vec![&float1, &float2, &sum1, &sum2];
        let flow_nodes: Vec<_> = core_nodes
            .iter()
            .map(|x| FlowNode::from_core_node(*x))
            .collect();

        connect(&flow_nodes[2], 0, &flow_nodes[0], 0);
        connect(&flow_nodes[2], 1, &flow_nodes[1], 0);
        connect(&flow_nodes[3], 0, &flow_nodes[0], 0);
        connect(&flow_nodes[3], 1, &flow_nodes[2], 0);

        let order = TopologicalOrder::generate(&flow_nodes[3]);

        let float1_index = order.iter().position(|r| r == &flow_nodes[0]).unwrap();
        let float2_index = order.iter().position(|r| r == &flow_nodes[1]).unwrap();
        let sum1_index = order.iter().position(|r| r == &flow_nodes[2]).unwrap();
        let sum2_index = order.iter().position(|r| r == &flow_nodes[3]).unwrap();

        assert_eq!(order.len(), 4);
        assert!(float1_index < sum1_index);
        assert!(float2_index < sum1_index);
        assert!(sum1_index < sum2_index);
    }
}
