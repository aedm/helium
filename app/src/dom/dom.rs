use crate::render_graph::render_thread::RenderGraph;
use crate::render_graph::node_descriptor::NodeId;
use crate::dom::dom_element::{DomElement, DomElementRef, DomProviderRef, DomSlotRef};
use std::collections::HashMap;

pub struct Dom {
    pub flow_nodes: HashMap<NodeId, DomElementRef>,
    pub flow_root: DomElementRef,
}

impl Dom {
    pub fn new(core_dom: &RenderGraph) -> Dom {
        Dom {
            flow_root: DomElement::from_node(&core_dom.core_root),
            flow_nodes: Default::default(),
        }
    }

    pub fn add_flow_node(&mut self, flow_node: &DomElementRef) {
        self.flow_nodes
            .insert(flow_node.borrow().id, flow_node.clone());
    }

    pub fn remove_flow_node(&mut self, flow_node: &DomElementRef) {
        self.flow_nodes.remove(&flow_node.borrow().id);
    }

    pub fn add_slot_to_provider(
        &mut self,
        provider_ref: &DomProviderRef,
        slot_ref: &DomSlotRef,
    ) {
        let mut node = provider_ref.node.borrow_mut();
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
