use crate::dom::flow_node::{ElementProviderRef, ElementSlotRef};
use crate::dom::mutation::FlowMutation;
use crate::dom::mutation_create_node::CreateNodeFlowMutation;
use crate::dom::mutation_remove_node::RemoveNodeFlowMutation;
use crate::dom::mutation_set_connections::SetSlotConnectionsFlowMutation;
use crate::stillaxis::Stillaxis;
use stillaxis_core::nodes::float_node::FloatNode;
use stillaxis_core::nodes::sum_node::SumNode;
use stillaxis_core::provider::ProviderValue;
use stillaxis_core::render::render_graph::Message;

mod dom;
mod stillaxis;

fn get_incoming(stillaxis: &mut Stillaxis) -> Box<Message> {
    stillaxis
        .core_dom
        .receiver_from_render_thread
        .recv()
        .unwrap()
}

fn assert_mutation_response(stillaxis: &mut Stillaxis) {
    let message = get_incoming(stillaxis);
    assert!(matches!(message.as_ref(), Message::Mutate { .. }));
}

fn assert_value_response(stillaxis: &mut Stillaxis, value: &ProviderValue) {
    let message = get_incoming(stillaxis);
    match message.as_ref() {
        Message::GetProviderValue(value_request) => {
            assert_eq!(value_request.response_value.as_ref().unwrap(), value);
        }
        _ => panic!(),
    }
}

fn main() {
    let mut stillaxis = Stillaxis::new();

    let mut _c1;
    let csum;
    let fsum = stillaxis.new_node::<SumNode>();
    {
        let ff1 = stillaxis.new_node::<FloatNode>();

        _c1 = Some(ff1.borrow().core_node.clone());
        csum = fsum.borrow().core_node.clone();

        let mut flow_mutation = FlowMutation::new(vec![
            CreateNodeFlowMutation::new(&ff1),
            CreateNodeFlowMutation::new(&fsum),
            SetSlotConnectionsFlowMutation::new(
                ElementSlotRef::new(&stillaxis.get_root(), "all_nodes"),
                vec![ElementProviderRef::new(&fsum, "node")],
            ),
            SetSlotConnectionsFlowMutation::new(
                ElementSlotRef::new(&fsum, "a"),
                vec![ElementProviderRef::new(&ff1, "value")],
            ),
        ]);

        stillaxis.run_mutation(&mut flow_mutation);
        assert_mutation_response(&mut stillaxis);
        assert!(_c1.as_ref().unwrap().refc() > 1);
        assert!(csum.refc() > 1);

        stillaxis.send_value_request(&ElementProviderRef::new(&fsum, "sum"));
        assert_value_response(&mut stillaxis, &ProviderValue::Float32(0.0));

        let mut flow_mutation = FlowMutation::new(vec![
            SetSlotConnectionsFlowMutation::new(ElementSlotRef::new(&fsum, "a"), vec![]),
            RemoveNodeFlowMutation::new(&ff1),
        ]);
        stillaxis.run_mutation(&mut flow_mutation);
    }
    // TODO: last reference should be held by mutation object
    assert!(_c1.as_ref().unwrap().refc() > 1);
    assert_mutation_response(&mut stillaxis);
    assert_eq!(_c1.as_ref().unwrap().refc(), 1);
    assert!(csum.refc() > 1);
    _c1 = None;
}
