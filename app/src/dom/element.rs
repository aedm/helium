use crate::render_graph::node_descriptor::NodeId;
use crate::render_graph::node_ref::NodeRef;
use crate::render_graph::provider::Provider;
use crate::render_graph::rcell::RCell;
use crate::render_graph::slot::Slot;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

pub struct Element {
    pub id: NodeId,
    pub name: String,
    pub key: String,
    pub core_node: NodeRef,
    pub slots: Vec<DomSlot>,
    pub providers: Vec<DomProvider>,
}

pub type DomElementRef = RCell<Element>;

pub struct DomSlot {
    pub name: String,
    pub connections: Vec<DomProviderRef>,
}

impl DomSlot {
    fn from_core_slot(core_slot: &Slot) -> DomSlot {
        DomSlot {
            name: core_slot.name.clone(),
            // TODO: initialize connections
            connections: Vec::new(),
        }
    }
}

pub struct DomProvider {
    pub name: String,
    pub connections: Vec<DomSlotRef>,
}

impl DomProvider {
    fn from_core_provider(core_provider: &Provider) -> DomProvider {
        DomProvider {
            name: core_provider.name.clone(),
            // TODO: initialize connections
            connections: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct DomSlotRef {
    pub element: DomElementRef,
    pub slot_index: usize,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct DomProviderRef {
    pub element: DomElementRef,
    pub provider_index: usize,
}

impl Element {
    pub fn from_node(core_node_ref: &NodeRef) -> DomElementRef {
        let core_node = core_node_ref.borrow_mut();
        let slots: Vec<_> = core_node
            .descriptor()
            .slots
            .iter()
            .map(|x| DomSlot::from_core_slot(&x.borrow()))
            .collect();
        let providers: Vec<_> = core_node
            .descriptor()
            .providers
            .iter()
            .map(|x| DomProvider::from_core_provider(&x.borrow()))
            .collect();
        RCell::new(Element {
            id: core_node.descriptor().id,
            name: core_node.descriptor().name.clone(),
            key: "".into(),
            core_node: core_node_ref.clone(),
            slots,
            providers,
        })
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("'{}'({})", self.name, self.id))
    }
}

impl Drop for Element {
    fn drop(&mut self) {
        println!(
            "Flow node drop: {:?}, render_graph refcount: {}",
            self,
            self.core_node.refc()
        );
    }
}

impl DomSlotRef {
    pub fn new(node: &DomElementRef, name: &str) -> DomSlotRef {
        if let Some(index) = node.borrow().slots.iter().position(|x| x.name == name) {
            return DomSlotRef {
                element: node.clone(),
                slot_index: index,
            };
        }
        panic!("Slot not found: '{}', node {:?}", name, node);
    }
}

impl DomProviderRef {
    pub fn new(node_ref: &DomElementRef, name: &str) -> DomProviderRef {
        let node = node_ref.borrow();
        if let Some(index) = node.providers.iter().position(|x| x.name == name) {
            return DomProviderRef {
                element: node_ref.clone(),
                provider_index: index,
            };
        }
        panic!("Provider not found: '{}', node {:?}", name, node);
    }
}
