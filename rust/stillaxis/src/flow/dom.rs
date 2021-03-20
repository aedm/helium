use crate::flow::flow_node::{FlowNode, FlowNodeRef, FlowProviderIndex, FlowSlotIndex};
use std::collections::HashMap;
use stillaxis_core::core_node_descriptor::NodeId;
use stillaxis_core::core_dom::CoreDom;

pub struct FlowDom {
    pub flow_nodes: HashMap<NodeId, FlowNodeRef>,
    pub flow_root: FlowNodeRef,
}

impl FlowDom {
    pub fn new(core_dom: &CoreDom) -> FlowDom {
        FlowDom {
            flow_root: FlowNode::from_core_node(&core_dom.core_root),
            flow_nodes: Default::default(),
        }
    }

    pub fn add_flow_node(&mut self, flow_node: &FlowNodeRef) {
        self.flow_nodes
            .insert(flow_node.borrow().id, flow_node.clone());
    }

    pub fn remove_flow_node(&mut self, flow_node: &FlowNodeRef) {
        self.flow_nodes.remove(&flow_node.borrow().id);
    }

    pub fn add_slot_to_provider(
        &mut self,
        provider_ref: &FlowProviderIndex,
        slot_ref: &FlowSlotIndex,
    ) {
        let mut node = provider_ref.node.borrow_mut();
        let provider = &mut node.providers[provider_ref.provider_index];
        if !provider.connections.contains(slot_ref) {
            provider.connections.push(slot_ref.clone());
        }
    }

    pub fn remove_slot_from_provider(
        &mut self,
        provider_ref: &FlowProviderIndex,
        slot_ref: &FlowSlotIndex,
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
