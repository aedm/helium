use crate::core::node::CoreNode;
use crate::flow::dom::Dom;
use crate::flow::flow_node::FlowNode;
use crate::flow::mutation::FlowMutation;
use crate::flow::mutation_create_node::CreateNodeFlowMutation;
use crate::flow::mutation_set_connections::SetSlotConnectionsFlowMutation;
use crate::nodes::float_node::FloatNode;
use crate::nodes::sum_node::SumNode;

mod core;
mod flow;
mod nodes;
mod stillaxis;

fn main() {
    let mut dom = Dom::new();

    let cf1 = CoreNode::new::<FloatNode>();
    let cf2 = CoreNode::new::<FloatNode>();
    let csum = CoreNode::new::<SumNode>();

    let ff1 = FlowNode::from_core_node(&cf1);
    let ff2 = FlowNode::from_core_node(&cf2);
    let fsum = FlowNode::from_core_node(&csum);

    let mut flow_mutation = FlowMutation::new(vec![
        CreateNodeFlowMutation::new(&ff1),
        CreateNodeFlowMutation::new(&ff2),
        CreateNodeFlowMutation::new(&fsum),
        SetSlotConnectionsFlowMutation::new_single(&fsum, 0, &ff1, 0),
        SetSlotConnectionsFlowMutation::new_single(&fsum, 1, &ff2, 0),
    ]);

    let mut core_mutation = flow_mutation.run(&mut dom);
    core_mutation.run();

    csum.borrow_mut().run_deps();

    println!("{:?}", csum.borrow().providers[0].borrow().provider_value);
}
