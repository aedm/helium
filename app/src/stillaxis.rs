use crate::render_graph::render_thread::MessageToRenderThread::{GetProviderValue, Mutate};
use crate::render_graph::render_thread::{RenderGraph, ProviderValueRequest};
use crate::render_graph::node::{Node, NodeProviderRef};
use crate::dom::dom::Dom;
use crate::dom::dom_element::{DomElement, DomElementRef, DomProviderRef};
use crate::dom::mutation::DomMutation;

pub struct Stillaxis {
    pub render_graph: RenderGraph,
    pub dom: Dom,
}

impl Stillaxis {
    pub fn new() -> Stillaxis {
        let render_graph = RenderGraph::new();
        let dom = Dom::new(&render_graph);

        Stillaxis { render_graph, dom }
    }

    pub fn new_node<T: 'static + Node>(&self) -> DomElementRef {
        let core_node = self.render_graph.new_node::<T>();
        DomElement::from_node(&core_node)
    }

    pub fn run_mutation(&mut self, flow_mutation: &mut DomMutation) {
        let core_mutation = flow_mutation.run(&mut self.dom);
        let _ = self
            .render_graph
            .sender_to_render_thread
            .send(Box::new(Mutate(core_mutation)));
    }

    pub fn get_root(&self) -> DomElementRef {
        self.dom.flow_root.clone()
    }

    pub fn send_value_request(&mut self, provider_ref: &DomProviderRef) {
        let request: ProviderValueRequest = ProviderValueRequest {
            provider: NodeProviderRef {
                node: provider_ref.node.borrow().core_node.clone(),
                provider_index: provider_ref.provider_index,
            },
            response_value: None,
        };
        let _ = self
            .render_graph
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
    use crate::render_graph::render_thread::MessageToRenderThread;
    use crate::render_graph::provider::ProviderValue;
    use crate::render_graph::slot::SlotDefault;
    use crate::dom::dom_element::{DomProviderRef, DomSlotRef};
    use crate::dom::mutation::DomMutation;
    use crate::dom::mutation_create_node::CreateNodeDomMutation;
    use crate::dom::mutation_remove_node::RemoveNodeDomMutation;
    use crate::dom::mutation_set_connections::SetSlotConnectionsDomMutation;
    use crate::dom::mutation_set_slot_value::SetSlotValueDomMutation;
    use crate::nodes::float_node::FloatNode;
    use crate::nodes::sum_node::SumNode;
    use crate::stillaxis::Stillaxis;

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

    #[test]
    fn simple_sum_graph() {
        let mut stillaxis = Stillaxis::new();

        let ff1 = stillaxis.new_node::<FloatNode>();
        let ff2 = stillaxis.new_node::<FloatNode>();
        let fsum = stillaxis.new_node::<SumNode>();

        let mut flow_mutation = DomMutation::new(vec![
            CreateNodeDomMutation::new(&ff1),
            CreateNodeDomMutation::new(&ff2),
            CreateNodeDomMutation::new(&fsum),
            SetSlotConnectionsDomMutation::new(
                DomSlotRef::new(&fsum, "a"),
                vec![DomProviderRef::new(&ff1, "value")],
            ),
            SetSlotConnectionsDomMutation::new(
                DomSlotRef::new(&fsum, "b"),
                vec![DomProviderRef::new(&ff2, "value")],
            ),
            SetSlotConnectionsDomMutation::new(
                DomSlotRef::new(&stillaxis.get_root(), "all_nodes"),
                vec![DomProviderRef::new(&fsum, "node")],
            ),
        ]);

        // thread::sleep(Duration::from_millis(100));
        stillaxis.run_mutation(&mut flow_mutation);
        assert_mutation_response(&mut stillaxis);

        stillaxis.send_value_request(&DomProviderRef::new(&fsum, "sum"));
        assert_value_response(&mut stillaxis, &ProviderValue::Float32(0.0));

        let mut flow_mutation = DomMutation::new(vec![SetSlotValueDomMutation::_new(
            &ff1,
            "a",
            SlotDefault::Float32(10.0),
        )]);
        // thread::sleep(Duration::from_millis(100));
        stillaxis.run_mutation(&mut flow_mutation);
        assert_mutation_response(&mut stillaxis);

        stillaxis.send_value_request(&DomProviderRef::new(&fsum, "sum"));
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
        assert!(_c1.as_ref().unwrap().refc() > 1);
        assert_mutation_response(&mut stillaxis);
        assert_eq!(_c1.as_ref().unwrap().refc(), 1);
        assert!(csum.refc() > 1);
        _c1 = None;
    }
}
