use crate::core::core_dom::CoreMessage::{GetProviderValue, Mutate};
use crate::core::core_dom::{CoreDom, ProviderValueRequest};
use crate::core::node::{CoreNode, CoreProviderIndex};
use crate::flow::dom::FlowDom;
use crate::flow::flow_node::{FlowNode, FlowNodeRef, FlowProviderIndex};
use crate::flow::mutation::FlowMutation;
use crate::render::vulkan::{VulkanContext, VulkanWindow};

pub struct Stillaxis {
    pub core_dom: CoreDom,
    pub flow_dom: FlowDom,
    pub vulkan_context: Option<VulkanContext>,
    pub vulkan_windows: Vec<VulkanWindow>,
}

impl Stillaxis {
    pub fn new() -> Stillaxis {
        let core_dom = CoreDom::new();
        let flow_dom = FlowDom::new(&core_dom);

        Stillaxis {
            core_dom,
            flow_dom,
            vulkan_context: None,
            vulkan_windows: vec![],
        }
    }

    pub fn initialize_vulkan_context(&mut self) {
        self.vulkan_context = Some(VulkanContext::new());
    }

    pub fn create_vulkan_window(&mut self) -> VulkanWindow {
        let window = VulkanWindow::new(&self.vulkan_context.as_ref().unwrap());
        self.vulkan_windows.push(window.clone());
        window
    }

    pub fn new_node<T: 'static + CoreNode>(&self) -> FlowNodeRef {
        let core_node = self.core_dom.new_node::<T>();
        FlowNode::from_core_node(&core_node)
    }

    pub fn run_mutation(&mut self, flow_mutation: &mut FlowMutation) {
        let core_mutation = flow_mutation.run(&mut self.flow_dom);
        let _ = self
            .core_dom
            .sender_to_render_thread
            .send(Box::new(Mutate(core_mutation)));
    }

    pub fn get_root(&self) -> FlowNodeRef {
        self.flow_dom.flow_root.clone()
    }

    pub fn send_value_request(&mut self, provider_ref: &FlowProviderIndex) {
        let request: ProviderValueRequest = ProviderValueRequest {
            provider: CoreProviderIndex {
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
                FlowSlotIndex::new(&fsum, "a"),
                vec![FlowProviderIndex::new(&ff1, "value")],
            ),
            SetSlotConnectionsFlowMutation::new(
                FlowSlotIndex::new(&fsum, "b"),
                vec![FlowProviderIndex::new(&ff2, "value")],
            ),
            SetSlotConnectionsFlowMutation::new(
                FlowSlotIndex::new(&stillaxis.get_root(), "all_nodes"),
                vec![FlowProviderIndex::new(&fsum, "node")],
            ),
        ]);

        // thread::sleep(Duration::from_millis(100));
        stillaxis.run_mutation(&mut flow_mutation);
        assert_mutation_response(&mut stillaxis);

        stillaxis.send_value_request(&FlowProviderIndex::new(&fsum, "sum"));
        assert_value_response(&mut stillaxis, &CoreProviderValue::Float32(0.0));

        let mut flow_mutation = FlowMutation::new(vec![SetSlotValueFlowMutation::_new(
            &ff1,
            "a",
            CoreSlotDefault::Float32(10.0),
        )]);
        // thread::sleep(Duration::from_millis(100));
        stillaxis.run_mutation(&mut flow_mutation);
        assert_mutation_response(&mut stillaxis);

        stillaxis.send_value_request(&FlowProviderIndex::new(&fsum, "sum"));
        assert_value_response(&mut stillaxis, &CoreProviderValue::Float32(10.0));
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
                    FlowSlotIndex::new(&stillaxis.get_root(), "all_nodes"),
                    vec![FlowProviderIndex::new(&fsum, "node")],
                ),
                SetSlotConnectionsFlowMutation::new(
                    FlowSlotIndex::new(&fsum, "a"),
                    vec![FlowProviderIndex::new(&ff1, "value")],
                ),
            ]);

            stillaxis.run_mutation(&mut flow_mutation);
            assert_mutation_response(&mut stillaxis);
            assert!(_c1.as_ref().unwrap().refc() > 1);
            assert!(csum.refc() > 1);

            stillaxis.send_value_request(&FlowProviderIndex::new(&fsum, "sum"));
            assert_value_response(&mut stillaxis, &CoreProviderValue::Float32(0.0));

            let mut flow_mutation = FlowMutation::new(vec![
                SetSlotConnectionsFlowMutation::new(FlowSlotIndex::new(&fsum, "a"), vec![]),
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
