use crate::core::node::{CoreNode, CoreNodeRef, NodeId};
use crate::core::provider::CoreProvider;
use crate::core::rf::ACell;
use crate::core::slot::CoreSlot;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

static NODE_ID_GENERATOR: AtomicU64 = AtomicU64::new(1);

enum ProviderType {
    None,
    Float32,
}

pub struct FlowNode {
    pub id: NodeId,
    pub key: String,
    pub core_node: CoreNodeRef,
    pub slots: Vec<FlowSlot>,
    pub providers: Vec<FlowProvider>,
}

pub type FlowNodeRef = Rc<RefCell<FlowNode>>;

pub struct FlowSlot {
    pub connections: Vec<FlowSlotIndex>,
}

impl FlowSlot {
    fn from_core_slot(core_slot: &CoreSlot) -> FlowSlot {
        FlowSlot {
            connections: Vec::new(),
        }
    }
}

pub struct FlowProvider {
    connections: Vec<FlowSlotIndex>,
}

impl FlowProvider {
    fn from_core_provider(core_provider: &CoreProvider) -> FlowProvider {
        FlowProvider {
            connections: Vec::new(),
        }
    }
}

#[derive(Clone)]
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
        let mut core_node = core_node_ref.borrow_mut();
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
        Rc::new(RefCell::new(FlowNode {
            id: NODE_ID_GENERATOR.fetch_add(1, Ordering::Relaxed),
            key: "".into(),
            core_node: core_node_ref.clone(),
            slots,
            providers,
        }))
    }
}
