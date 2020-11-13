use crate::core::core_dom::CoreMessage;
use crate::core::slot::CoreSlotDefault::Float32;
use crate::flow::mutation::FlowMutation;
use crate::flow::mutation_create_node::CreateNodeFlowMutation;
use crate::flow::mutation_set_connections::SetSlotConnectionsFlowMutation;
use crate::flow::mutation_set_slot_value::SetSlotValueFlowMutation;
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
    thread::sleep(Duration::from_millis(1000));
}
