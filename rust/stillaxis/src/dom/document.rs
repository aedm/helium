use crate::dom::flow_node::{Element, ElementProviderRef, ElementRef, ElementSlotRef};
use std::collections::HashMap;
use stillaxis_core::node_descriptor::NodeId;
use stillaxis_core::render::render_graph::RenderGraph;

pub struct Document {
    pub elements: HashMap<NodeId, ElementRef>,
    pub root: ElementRef,
}

impl Document {
    pub fn new(render_graph: &RenderGraph) -> Document {
        Document {
            root: Element::from_core_node(&render_graph.root_node),
            elements: Default::default(),
        }
    }

    pub fn add_component(&mut self, flow_node: &ElementRef) {
        self.elements
            .insert(flow_node.borrow().id, flow_node.clone());
    }

    pub fn remove_component(&mut self, flow_node: &ElementRef) {
        self.elements.remove(&flow_node.borrow().id);
    }

    pub fn add_slot_to_provider(
        &mut self,
        provider_ref: &ElementProviderRef,
        slot_ref: &ElementSlotRef,
    ) {
        let mut node = provider_ref.node.borrow_mut();
        let provider = &mut node.providers[provider_ref.provider_index];
        if !provider.connections.contains(slot_ref) {
            provider.connections.push(slot_ref.clone());
        }
    }

    pub fn remove_slot_from_provider(
        &mut self,
        provider_ref: &ElementProviderRef,
        slot_ref: &ElementSlotRef,
    ) {
        let mut node = provider_ref.node.borrow_mut();
        let provider = &mut node.providers[provider_ref.provider_index];
        let position = provider
            .connections
            .iter()
            .position(|x| x == slot_ref)
            .expect("slot not found");
        provider.connections.remove(position);
    }
}
