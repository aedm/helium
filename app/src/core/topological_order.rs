use crate::core::node::CoreNodeRef;
use std::collections::HashSet;

pub struct TopologicalOrder {
    visited: HashSet<CoreNodeRef>,
    order: Vec<CoreNodeRef>,
}

impl TopologicalOrder {
    pub fn generate(node_ref: &CoreNodeRef) -> Vec<CoreNodeRef> {
        let mut order = TopologicalOrder {
            visited: HashSet::new(),
            order: Vec::new(),
        };
        order.visit(node_ref);
        order.order
    }

    fn visit(&mut self, node_ref: &CoreNodeRef) {
        if !self.visited.insert(node_ref.clone()) {
            return;
        }
        for slot_ref in &node_ref.borrow().slots {
            let slot = &slot_ref.borrow();
            for provider_ref in &slot.connection {
                self.visit(&provider_ref.borrow().owner.upgrade().unwrap());
            }
        }
        self.order.push(node_ref.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::core::node::CoreNode;
    use crate::core::slot::connect_slot;
    use crate::core::topological_order::TopologicalOrder;
    use crate::nodes::float_node::FloatNode;
    use crate::nodes::sum_node::SumNode;

    #[test]
    fn generates_correct_topological_order() {
        let float1 = CoreNode::new::<FloatNode>();
        let float2 = CoreNode::new::<FloatNode>();
        let sum1 = CoreNode::new::<SumNode>();
        let sum2 = CoreNode::new::<SumNode>();

        connect_slot(
            &sum1.borrow_mut().slots[0],
            &float1.borrow_mut().providers[0],
        );
        connect_slot(
            &sum1.borrow_mut().slots[1],
            &float2.borrow_mut().providers[0],
        );
        connect_slot(
            &sum2.borrow_mut().slots[0],
            &float2.borrow_mut().providers[0],
        );
        connect_slot(&sum2.borrow_mut().slots[1], &sum1.borrow_mut().providers[0]);

        let order = TopologicalOrder::generate(&sum2);

        let float1_index = order.iter().position(|r| r == &float1).unwrap();
        let float2_index = order.iter().position(|r| r == &float2).unwrap();
        let sum1_index = order.iter().position(|r| r == &sum1).unwrap();
        let sum2_index = order.iter().position(|r| r == &sum2).unwrap();

        assert_eq!(order.len(), 4);
        assert!(float1_index < sum1_index);
        assert!(float2_index < sum1_index);
        assert!(sum1_index < sum2_index);
    }
}
