pub mod document;
pub mod flow_node;
pub mod mutation;
pub mod mutation_create_node;
pub mod mutation_remove_node;
pub mod mutation_set_connections;
pub mod mutation_set_slot_value;
pub mod topological_order;

#[cfg(test)]
mod module_tests {
    // use crate::dom::dom::FlowDom;
    // use crate::dom::flow_node::FlowNode;
    // use crate::dom::mutation::FlowMutation;
    // use crate::dom::mutation_create_node::CreateNodeFlowMutation;
    // use crate::dom::mutation_set_connections::SetSlotConnectionsFlowMutation;
    // use crate::nodes::float_node::FloatNode;
    // use crate::nodes::sum_node::SumNode;

    // #[test]
    // fn generates_simple_sum_graph() {
    //     let mut dom = FlowDom::new();
    //
    //     let cf1 = CoreNode::new::<FloatNode>();
    //     let cf2 = CoreNode::new::<FloatNode>();
    //     let csum = CoreNode::new::<SumNode>();
    //
    //     let ff1 = FlowNode::from_core_node(&cf1);
    //     let ff2 = FlowNode::from_core_node(&cf2);
    //     let fsum = FlowNode::from_core_node(&csum);
    //
    //     let mut flow_mutation = FlowMutation::new(vec![
    //         CreateNodeFlowMutation::new(&ff1),
    //         CreateNodeFlowMutation::new(&ff2),
    //         CreateNodeFlowMutation::new(&fsum),
    //         SetSlotConnectionsFlowMutation::new_single(&fsum, 0, &ff1, 0),
    //         SetSlotConnectionsFlowMutation::new_single(&fsum, 1, &ff2, 0),
    //     ]);
    //
    //     let mut core_mutation = flow_mutation.run(&mut dom);
    //     core_mutation.run();
    //
    //     csum.borrow_mut().run_deps();
    //
    //     println!("{:?}", csum.borrow().providers[0].borrow().provider_value);
    // }
}
