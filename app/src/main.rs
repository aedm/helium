use crate::render_graph::render_thread::MessageToRenderThread;
use crate::render_graph::provider::ProviderValue;
use crate::dom::dom_element::{DomProviderRef, DomSlotRef};
use crate::dom::mutation::DomMutation;
use crate::dom::mutation_create_node::CreateNodeDomMutation;
use crate::dom::mutation_remove_node::RemoveNodeDomMutation;
use crate::dom::mutation_set_connections::SetSlotConnectionsDomMutation;
use crate::nodes::float_node::FloatNode;
use crate::nodes::sum_node::SumNode;
use crate::stillaxis::Stillaxis;

mod render_graph;
mod dom;
mod nodes;
mod providers;
mod slots;
mod stillaxis;

fn get_incoming(stillaxis: &mut Stillaxis) -> Box<MessageToRenderThread> {
    stillaxis
        .render_graph
        .receiver_from_render_thread
        .recv()
        .unwrap()
}

fn assert_mutation_response(stillaxis: &mut Stillaxis) {
    let message = get_incoming(stillaxis);
    assert!(matches!(message.as_ref(), MessageToRenderThread::Mutate { .. }));
}

fn assert_value_response(stillaxis: &mut Stillaxis, value: &ProviderValue) {
    let message = get_incoming(stillaxis);
    match message.as_ref() {
        MessageToRenderThread::GetProviderValue(value_request) => {
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

        let mut flow_mutation = DomMutation::new(vec![
            CreateNodeDomMutation::new(&ff1),
            CreateNodeDomMutation::new(&fsum),
            SetSlotConnectionsDomMutation::new(
                DomSlotRef::new(&stillaxis.get_root(), "all_nodes"),
                vec![DomProviderRef::new(&fsum, "node")],
            ),
            SetSlotConnectionsDomMutation::new(
                DomSlotRef::new(&fsum, "a"),
                vec![DomProviderRef::new(&ff1, "value")],
            ),
        ]);

        stillaxis.run_mutation(&mut flow_mutation);
        assert_mutation_response(&mut stillaxis);
        assert!(_c1.as_ref().unwrap().refc() > 1);
        assert!(csum.refc() > 1);

        stillaxis.send_value_request(&DomProviderRef::new(&fsum, "sum"));
        assert_value_response(&mut stillaxis, &ProviderValue::Float32(0.0));

        let mut flow_mutation = DomMutation::new(vec![
            SetSlotConnectionsDomMutation::new(DomSlotRef::new(&fsum, "a"), vec![]),
            RemoveNodeDomMutation::new(&ff1),
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
