use crate::render_graph::render_thread::RenderGraph;
use crate::render_graph::node_descriptor::NodeId;
use crate::dom::element::{Element, DomElementRef, DomProviderRef, DomSlotRef};
use std::collections::HashMap;

pub struct Dom {
    pub elements: HashMap<NodeId, DomElementRef>,
    pub root_element: DomElementRef,
}

impl Dom {
    pub fn new(render_graph: &RenderGraph) -> Dom {
        Dom {
            elements: Default::default(),
            root_element: Element::from_node(&render_graph.root_node),
        }
    }

    pub fn add_flow_node(&mut self, flow_node: &DomElementRef) {
        self.elements
            .insert(flow_node.borrow().id, flow_node.clone());
    }

    pub fn remove_flow_node(&mut self, flow_node: &DomElementRef) {
        self.elements.remove(&flow_node.borrow().id);
    }

    pub fn add_slot_to_provider(
        &mut self,
        provider_ref: &DomProviderRef,
        slot_ref: &DomSlotRef,
    ) {
        let mut node = provider_ref.element.borrow_mut();
        let provider = &mut node.providers[provider_ref.provider_index];
        if !provider.connections.contains(slot_ref) {
            provider.connections.push(slot_ref.clone());
        }
    }

    pub fn remove_slot_from_provider(
        &mut self,
        provider_ref: &DomProviderRef,
        slot_ref: &DomSlotRef,
    ) {
        let mut node = provider_ref.element.borrow_mut();
        let provider = &mut node.providers[provider_ref.provider_index];
        let position = provider
            .connections
            .iter()
            .position(|x| x == slot_ref)
            .expect("slot not found");
        provider.connections.remove(position);
    }
}
