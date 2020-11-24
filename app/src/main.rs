use crate::core::core_dom::CoreMessage;
use crate::core::provider::CoreProviderValue;
use crate::flow::flow_node::{FlowProviderIndex, FlowSlotIndex};
use crate::flow::mutation::FlowMutation;
use crate::flow::mutation_create_node::CreateNodeFlowMutation;
use crate::flow::mutation_remove_node::RemoveNodeFlowMutation;
use crate::flow::mutation_set_connections::SetSlotConnectionsFlowMutation;
use crate::nodes::float_node::FloatNode;
use crate::nodes::sum_node::SumNode;
use crate::stillaxis::Stillaxis;
use std::thread;
use std::time::Duration;

mod core;
mod flow;
mod nodes;
mod providers;
mod slots;
mod stillaxis;
mod render;

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
            assert_eq!(value_request.response_value.as_ref().unwrap(), value);
        }
        _ => panic!(),
    }
}

fn perform_test(stillaxis: &mut Stillaxis) {
    let csum;
    let mut _c1;
    let fsum = stillaxis.new_node::<SumNode>();
    {
        let ff1 = stillaxis.new_node::<FloatNode>();

        _c1 = Some(ff1.borrow().core_node.clone());
        csum = fsum.borrow().core_node.clone();

        let mut flow_mutation = FlowMutation::new(vec![
            CreateNodeFlowMutation::new(&ff1),
            CreateNodeFlowMutation::new(&fsum),
            SetSlotConnectionsFlowMutation::new(
                FlowSlotIndex::new(&stillaxis.get_root(), "all_nodes"),
                vec![FlowProviderIndex::new(&fsum, "node")],
            ),
            SetSlotConnectionsFlowMutation::new(
                FlowSlotIndex::new(&fsum, "a"),
                vec![FlowProviderIndex::new(&ff1, "value")],
            ),
        ]);

        stillaxis.run_mutation(&mut flow_mutation);
        assert_mutation_response(stillaxis);
        assert!(_c1.as_ref().unwrap().refc() > 1);
        assert!(csum.refc() > 1);

        stillaxis.send_value_request(&FlowProviderIndex::new(&fsum, "sum"));
        assert_value_response(stillaxis, &CoreProviderValue::Float32(0.0));

        let mut flow_mutation = FlowMutation::new(vec![
            SetSlotConnectionsFlowMutation::new(FlowSlotIndex::new(&fsum, "a"), vec![]),
            RemoveNodeFlowMutation::new(&ff1),
        ]);
        stillaxis.run_mutation(&mut flow_mutation);
    }
    // TODO: last reference should be held by mutation object
    assert!(_c1.as_ref().unwrap().refc() > 1);
    assert_mutation_response(stillaxis);
    assert_eq!(_c1.as_ref().unwrap().refc(), 1);
    assert!(csum.refc() > 1);
    _c1 = None;
}

fn main_window(stillaxis: &mut Stillaxis) {
    stillaxis.initialize_vulkan_context();
    let window = stillaxis.create_vulkan_window();
    thread::sleep(Duration::from_millis(2000));
}

fn main() {
    let mut stillaxis = Stillaxis::new();
    main_window(&mut stillaxis);
    thread::sleep(Duration::from_millis(2000));
    perform_test(&mut stillaxis);
}
