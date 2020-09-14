use crate::flow::node::{Node, NodeRef};
use crate::flow::rf::Rf;
use std::collections::HashSet;

pub mod node;
pub mod provider;
pub mod rf;
pub mod slot;

struct TopologicalOrder {
    visited: HashSet<NodeRef>,
    order: Vec<NodeRef>,
}

impl TopologicalOrder {
    pub fn generate_topological_order(node_ref: &NodeRef) -> Vec<NodeRef> {
        let mut order = TopologicalOrder {
            visited: HashSet::new(),
            order: Vec::new(),
        };
        order.visit(node_ref);
        order.order
    }

    fn visit(&mut self, node_ref: &NodeRef) {
        self.visited.insert(node_ref.clone());

        self.order.push(node_ref.clone());
    }
}

fn generate_topological_order(rf: &NodeRef) -> Vec<Rf<Node>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use crate::flow::node::Node;
    use crate::nodes::float_node::FloatNode;
    use crate::nodes::sum_node::SumNode;
    use crate::flow::slot::connect_slot;
    use crate::flow::generate_topological_order;

    #[test]
    fn generates_correct_topological_order() {
        let float1 = Node::new::<FloatNode>();
        let float2 = Node::new::<FloatNode>();
        let sum1 = Node::new::<SumNode>();
        let sum2 = Node::new::<SumNode>();

        connect_slot(&sum1.borrow_mut().slots[0], &float1.borrow_mut().providers[0]);
        connect_slot(&sum1.borrow_mut().slots[1], &float2.borrow_mut().providers[0]);
        connect_slot(&sum2.borrow_mut().slots[0], &float2.borrow_mut().providers[0]);
        connect_slot(&sum2.borrow_mut().slots[1], &sum1.borrow_mut().providers[0]);

        let order = generate_topological_order(&sum2);

        let float1_index = order.iter().position(|r| r == &float1).unwrap();
        let float2_index = order.iter().position(|r| r == &float2).unwrap();
        let sum1_index = order.iter().position(|r| r == &sum1).unwrap();
        let sum2_index = order.iter().position(|r| r == &sum2).unwrap();

        assert!(float1_index < sum1_index);
        assert!(float2_index < sum1_index);
        assert!(sum1_index < sum2_index);
    }
}