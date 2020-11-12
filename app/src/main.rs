use crate::core::core_dom::CoreMessage;
use crate::core::slot::CoreSlotDefault::Float32;
use crate::flow::mutation::FlowMutation;
use crate::flow::mutation_create_node::CreateNodeFlowMutation;
use crate::flow::mutation_set_connections::SetSlotConnectionsFlowMutation;
use crate::flow::mutation_set_slot_value::SetSlotValueFlowMutation;
use crate::nodes::float_node::FloatNode;
use crate::nodes::sum_node::SumNode;
use crate::stillaxis::Stillaxis;

mod core;
mod flow;
mod nodes;
mod stillaxis;

fn get_incoming(stillaxis: &mut Stillaxis) {
    let message = stillaxis
        .core_dom
        .receiver_from_render_thread
        .recv()
        .unwrap();
    let inner = message.as_ref();
    match inner {
        CoreMessage::Stop => panic!(),
        CoreMessage::GetProviderValue(value) => {
            println!("<- Message: Value is {:?}", value.response_value);
        }
        _ => {
            println!("<- Message: {}", Into::<&str>::into(inner));
        }
    }
}

fn main() {
    let mut stillaxis = Stillaxis::new();

    let ff1 = stillaxis.new_node::<FloatNode>();
    let ff2 = stillaxis.new_node::<FloatNode>();
    let fsum = stillaxis.new_node::<SumNode>();

    let mut flow_mutation = FlowMutation::new(vec![
        CreateNodeFlowMutation::new(&ff1),
        CreateNodeFlowMutation::new(&ff2),
        CreateNodeFlowMutation::new(&fsum),
        SetSlotConnectionsFlowMutation::new_single(&fsum, 0, &ff1, 0),
        SetSlotConnectionsFlowMutation::new_single(&fsum, 1, &ff2, 0),
        SetSlotConnectionsFlowMutation::new_single(&stillaxis.get_root(), 0, &fsum, 0),
    ]);

    // thread::sleep(Duration::from_millis(100));
    stillaxis.run_mutation(&mut flow_mutation);
    get_incoming(&mut stillaxis);

    stillaxis.send_value_request(&fsum, 0);
    get_incoming(&mut stillaxis);

    let mut flow_mutation =
        FlowMutation::new(vec![SetSlotValueFlowMutation::new(&ff1, 0, Float32(10.0))]);
    // thread::sleep(Duration::from_millis(100));
    stillaxis.run_mutation(&mut flow_mutation);
    get_incoming(&mut stillaxis);

    stillaxis.send_value_request(&fsum, 0);
    get_incoming(&mut stillaxis);

    // thread::sleep(Duration::from_millis(900));
}
