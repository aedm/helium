use crate::core::core_dom::CoreMessage;
use crate::core::provider::CoreProviderValue;
use crate::core::slot::CoreSlotDefault;
use crate::flow::flow_node::{FlowProviderIndex, FlowSlotIndex};
use crate::flow::mutation::FlowMutation;
use crate::flow::mutation_create_node::CreateNodeFlowMutation;
use crate::flow::mutation_remove_node::RemoveNodeFlowMutation;
use crate::flow::mutation_set_connections::SetSlotConnectionsFlowMutation;
use crate::flow::mutation_set_slot_value::SetSlotValueFlowMutation;
use crate::nodes::float_node::FloatNode;
use crate::nodes::sum_node::SumNode;
use crate::stillaxis::Stillaxis;

mod core;
mod flow;
mod nodes;
mod providers;
mod slots;
mod stillaxis;

fn get_incoming(stillaxis: &mut Stillaxis) -> Box<CoreMessage> {
    stillaxis
        .core_dom
        .receiver_from_render_thread
        .recv()
        .unwrap()
}

fn assert_mutation_response(stillaxis: &mut Stillaxis) {
    let message = get_incoming(stillaxis);
    assert!(matches!(message.as_ref(), CoreMessage::Mutate { .. }));
}

fn assert_value_response(stillaxis: &mut Stillaxis, value: &CoreProviderValue) {
    let message = get_incoming(stillaxis);
    match message.as_ref() {
        CoreMessage::GetProviderValue(value_request) => {
            assert_eq!(value_request.response_value.unwrap(), *value);
        }
        _ => panic!(),
    }
}

fn main() {
    let mut stillaxis = Stillaxis::new();

    let fsum = stillaxis.new_node::<SumNode>();
    {
        let ff1 = stillaxis.new_node::<FloatNode>();
        let ff2 = stillaxis.new_node::<FloatNode>();
        let _ff3 = stillaxis.new_node::<FloatNode>();

        let mut flow_mutation = FlowMutation::new(vec![
            CreateNodeFlowMutation::new(&ff1),
            CreateNodeFlowMutation::new(&ff2),
            CreateNodeFlowMutation::new(&fsum),
            SetSlotConnectionsFlowMutation::new(
                FlowSlotIndex::new(&fsum, "a"),
                vec![FlowProviderIndex::new(&ff1, "value")],
            ),
            SetSlotConnectionsFlowMutation::new(
                FlowSlotIndex::new(&fsum, "b"),
                vec![FlowProviderIndex::new(&ff2, "value")],
            ),
            SetSlotConnectionsFlowMutation::new(
                FlowSlotIndex::new(&stillaxis.get_root(), "all_nodes"),
                vec![FlowProviderIndex::new(&fsum, "sum")],
            ),
            SetSlotValueFlowMutation::new(&ff1, "a", CoreSlotDefault::Float32(10.0)),
        ]);

        // thread::sleep(Duration::from_millis(100));
        stillaxis.run_mutation(&mut flow_mutation);
        assert_mutation_response(&mut stillaxis);

        stillaxis.send_value_request(&fsum, 0);
        assert_value_response(&mut stillaxis, &CoreProviderValue::Float32(10.0));

        let mut flow_mutation = FlowMutation::new(vec![
            SetSlotConnectionsFlowMutation::new(FlowSlotIndex::new(&fsum, "a"), vec![]),
            RemoveNodeFlowMutation::new(&ff1),
        ]);
        // println!("0 {:?}", stillaxis.flow_dom.flow_nodes.iter().collect::<Vec<_>>());
        stillaxis.run_mutation(&mut flow_mutation);
    }
    println!(
        "1 {:?}",
        stillaxis.flow_dom.flow_nodes.iter().collect::<Vec<_>>()
    );
    assert_mutation_response(&mut stillaxis);
    println!(
        "2 {:?}",
        stillaxis.flow_dom.flow_nodes.iter().collect::<Vec<_>>()
    );

    stillaxis.send_value_request(&fsum, 0);
    assert_value_response(&mut stillaxis, &CoreProviderValue::Float32(0.0));
}
