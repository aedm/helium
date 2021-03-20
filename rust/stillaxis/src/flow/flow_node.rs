use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use stillaxis_core::core_node_descriptor::NodeId;
use stillaxis_core::node_ref::CoreNodeRef;
use stillaxis_core::rcell::RCell;
use stillaxis_core::slot::CoreSlot;
use stillaxis_core::provider::CoreProvider;

pub struct FlowNode {
    pub id: NodeId,
    pub name: String,
    pub key: String,
    pub core_node: CoreNodeRef,
    pub slots: Vec<FlowSlot>,
    pub providers: Vec<FlowProvider>,
}

pub type FlowNodeRef = RCell<FlowNode>;

pub struct FlowSlot {
    pub name: String,
    pub connections: Vec<FlowProviderIndex>,
}

impl FlowSlot {
    fn from_core_slot(core_slot: &CoreSlot) -> FlowSlot {
        FlowSlot {
            name: core_slot.name.clone(),
            // TODO
            connections: Vec::new(),
        }
    }
}

pub struct FlowProvider {
    pub name: String,
    pub connections: Vec<FlowSlotIndex>,
}

impl FlowProvider {
    fn from_core_provider(core_provider: &CoreProvider) -> FlowProvider {
        FlowProvider {
            name: core_provider.name.clone(),
            // TODO
            connections: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FlowSlotIndex {
    pub node: FlowNodeRef,
    pub slot_index: usize,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FlowProviderIndex {
    pub node: FlowNodeRef,
    pub provider_index: usize,
}

impl FlowNode {
    pub fn from_core_node(core_node_ref: &CoreNodeRef) -> FlowNodeRef {
        let core_node = core_node_ref.borrow_mut();
        let slots: Vec<_> = core_node
            .descriptor()
            .slots
            .iter()
            .map(|x| FlowSlot::from_core_slot(&x.borrow()))
            .collect();
        let providers: Vec<_> = core_node
            .descriptor()
            .providers
            .iter()
            .map(|x| FlowProvider::from_core_provider(&x.borrow()))
            .collect();
        RCell::new(FlowNode {
            id: core_node.descriptor().id,
            name: core_node.descriptor().name.clone(),
            key: "".into(),
            core_node: core_node_ref.clone(),
            slots,
            providers,
        })
    }
}

impl Debug for FlowNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("'{}'({})", self.name, self.id))
    }
}

impl Drop for FlowNode {
    fn drop(&mut self) {
        println!(
            "Flow node drop: {:?}, core refcount: {}",
            self,
            self.core_node.refc()
        );
    }
}

impl FlowSlotIndex {
    pub fn new(node: &FlowNodeRef, name: &str) -> FlowSlotIndex {
        if let Some(index) = node.borrow().slots.iter().position(|x| x.name == name) {
            return FlowSlotIndex {
                node: node.clone(),
                slot_index: index,
            };
        }
        panic!("Slot not found: '{}', node {:?}", name, node);
    }
}

impl FlowProviderIndex {
    pub fn new(node_ref: &FlowNodeRef, name: &str) -> FlowProviderIndex {
        let node = node_ref.borrow();
        if let Some(index) = node.providers.iter().position(|x| x.name == name) {
            return FlowProviderIndex {
                node: node_ref.clone(),
                provider_index: index,
            };
        }
        panic!("Provider not found: '{}', node {:?}", name, node);
    }
}
