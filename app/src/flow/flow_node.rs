use crate::core::node::{CoreNodeRef, NodeId};
use crate::core::provider::CoreProvider;
use crate::core::rcell::RCell;
use crate::core::slot::CoreSlot;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};

static NODE_ID_GENERATOR: AtomicU64 = AtomicU64::new(1);

pub struct FlowNode {
    pub id: NodeId,
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
    pub connections: Vec<FlowSlotIndex>,
}

impl FlowProvider {
    fn from_core_provider(_core_provider: &CoreProvider) -> FlowProvider {
        FlowProvider {
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

#[derive(Clone)]
pub struct FlowProviderIndex {
    pub node: FlowNodeRef,
    pub provider_index: usize,
}

impl FlowNode {
    pub fn from_core_node(core_node_ref: &CoreNodeRef) -> FlowNodeRef {
        let core_node = core_node_ref.borrow_mut();
        let slots: Vec<_> = core_node
            .slots
            .iter()
            .map(|x| FlowSlot::from_core_slot(&x.borrow()))
            .collect();
        let providers: Vec<_> = core_node
            .providers
            .iter()
            .map(|x| FlowProvider::from_core_provider(&x.borrow()))
            .collect();
        RCell::new(FlowNode {
            id: NODE_ID_GENERATOR.fetch_add(1, Ordering::Relaxed),
            key: "".into(),
            core_node: core_node_ref.clone(),
            slots,
            providers,
        })
    }

    pub fn get_slot_by_name(node: &FlowNodeRef, name: &str) -> FlowSlotIndex {
        let index = node.borrow().slots.iter().position(|x| x.name == name).unwrap();
        FlowSlotIndex {
            node: node.clone(),
            slot_index: index,
        }
    }
}

impl Debug for FlowNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{}", self.id))
    }
}
