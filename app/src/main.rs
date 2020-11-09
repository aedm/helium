use crate::core::node::CoreNode;
use crate::flow::dom::FlowDom;
use crate::flow::flow_node::FlowNode;
use crate::flow::mutation::FlowMutation;
use crate::flow::mutation_create_node::CreateNodeFlowMutation;
use crate::flow::mutation_set_connections::SetSlotConnectionsFlowMutation;
use crate::nodes::float_node::FloatNode;
use crate::nodes::sum_node::SumNode;
use crate::stillaxis::Stillaxis;
use std::thread;
use std::time::Duration;

mod core;
mod flow;
mod nodes;
mod stillaxis;

fn main() {
    let mut stillaxis = Stillaxis::new();

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
        SetSlotConnectionsFlowMutation::new_single(&stillaxis.get_root(), 0, &fsum, 0),
    ]);

    stillaxis.run_mutation(&mut flow_mutation);

    thread::sleep(Duration::from_millis(1000));
    // core_mutation.run();
    //
    // csum.borrow_mut().run_deps();
    //
    // println!("{:?}", csum.borrow().providers[0].borrow().provider_value);
}
