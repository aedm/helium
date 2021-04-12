use crate::dom::document::Document;
use crate::dom::flow_node::{Element, ElementProviderRef, ElementRef};
use crate::dom::mutation::FlowMutation;
use stillaxis_core::node::{Node, ProviderRef};
use stillaxis_core::render::render_graph::Message::{GetProviderValue, Mutate};
use stillaxis_core::render::render_graph::{ProviderValueRequest, RenderGraph};

pub struct Stillaxis {
    pub core_dom: RenderGraph,
    pub flow_dom: Document,
}

impl Stillaxis {
    pub fn new() -> Stillaxis {
        let core_dom = RenderGraph::new();
        let flow_dom = Document::new(&core_dom);

        Stillaxis { core_dom, flow_dom }
    }

    pub fn new_node<T: 'static + Node>(&self) -> ElementRef {
        let core_node = self.core_dom.new_node::<T>();
        Element::from_core_node(&core_node)
    }

    pub fn run_mutation(&mut self, flow_mutation: &mut FlowMutation) {
        let core_mutation = flow_mutation.run(&mut self.flow_dom);
        let _ = self
            .core_dom
            .sender_to_render_thread
            .send(Box::new(Mutate(core_mutation)));
    }

    pub fn get_root(&self) -> ElementRef {
        self.flow_dom.root.clone()
    }

    pub fn send_value_request(&mut self, provider_ref: &ElementProviderRef) {
        let request: ProviderValueRequest = ProviderValueRequest {
            provider: ProviderRef {
                node: provider_ref.node.borrow().core_node.clone(),
                provider_index: provider_ref.provider_index,
            },
            response_value: None,
        };
        let _ = self
            .core_dom
            .sender_to_render_thread
            .send(Box::new(GetProviderValue(request)));
    }
}

impl Drop for Stillaxis {
    fn drop(&mut self) {
        dbg!("Stillaxis.drop");
    }
}

#[cfg(test)]
mod tests {
    use crate::dom::flow_node::{ElementProviderRef, ElementSlotRef};
    use crate::dom::mutation::FlowMutation;
    use crate::dom::mutation_create_node::CreateNodeFlowMutation;
    use crate::dom::mutation_remove_node::RemoveNodeFlowMutation;
    use crate::dom::mutation_set_connections::SetSlotConnectionsFlowMutation;
    use crate::dom::mutation_set_slot_value::SetSlotValueFlowMutation;
    use crate::stillaxis::Stillaxis;
    use stillaxis_core::nodes::float_node::FloatNode;
    use stillaxis_core::nodes::sum_node::SumNode;
    use stillaxis_core::provider::ProviderValue;
    use stillaxis_core::render::render_graph::Message;
    use stillaxis_core::slot::SlotDefaultValue;

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

    #[test]
    fn simple_sum_graph() {
        let mut stillaxis = Stillaxis::new();

        let ff1 = stillaxis.new_node::<FloatNode>();
        let ff2 = stillaxis.new_node::<FloatNode>();
        let fsum = stillaxis.new_node::<SumNode>();

        let mut flow_mutation = FlowMutation::new(vec![
            CreateNodeFlowMutation::new(&ff1),
            CreateNodeFlowMutation::new(&ff2),
            CreateNodeFlowMutation::new(&fsum),
            SetSlotConnectionsFlowMutation::new(
                ElementSlotRef::new(&fsum, "a"),
                vec![ElementProviderRef::new(&ff1, "value")],
            ),
            SetSlotConnectionsFlowMutation::new(
                ElementSlotRef::new(&fsum, "b"),
                vec![ElementProviderRef::new(&ff2, "value")],
            ),
            SetSlotConnectionsFlowMutation::new(
                ElementSlotRef::new(&stillaxis.get_root(), "all_nodes"),
                vec![ElementProviderRef::new(&fsum, "node")],
            ),
        ]);

        // thread::sleep(Duration::from_millis(100));
        stillaxis.run_mutation(&mut flow_mutation);
        assert_mutation_response(&mut stillaxis);

        stillaxis.send_value_request(&ElementProviderRef::new(&fsum, "sum"));
        assert_value_response(&mut stillaxis, &ProviderValue::Float32(0.0));

        let mut flow_mutation = FlowMutation::new(vec![SetSlotValueFlowMutation::_new(
            &ff1,
            "a",
            SlotDefaultValue::Float32(10.0),
        )]);
        // thread::sleep(Duration::from_millis(100));
        stillaxis.run_mutation(&mut flow_mutation);
        assert_mutation_response(&mut stillaxis);

        stillaxis.send_value_request(&ElementProviderRef::new(&fsum, "sum"));
        assert_value_response(&mut stillaxis, &ProviderValue::Float32(10.0));
    }

    #[test]
    fn no_dropping_nodes_on_render_thread() {
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
        assert!(_c1.as_ref().unwrap().refc() > 1);
        assert_mutation_response(&mut stillaxis);
        assert_eq!(_c1.as_ref().unwrap().refc(), 1);
        assert!(csum.refc() > 1);
        _c1 = None;
    }
}
