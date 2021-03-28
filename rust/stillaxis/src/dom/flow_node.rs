use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use stillaxis_core::node_descriptor::NodeId;
use stillaxis_core::node_ref::NodeRef;
use stillaxis_core::provider::Provider;
use stillaxis_core::rcell::RCell;
use stillaxis_core::slot::Slot;

pub struct Element {
    pub id: NodeId,
    pub name: String,
    pub key: String,
    pub core_node: NodeRef,
    pub slots: Vec<ElementSlot>,
    pub providers: Vec<ElementProvider>,
}

pub type ElementRef = RCell<Element>;

pub struct ElementSlot {
    pub name: String,
    pub connections: Vec<ElementProviderRef>,
}

impl ElementSlot {
    fn from_core_slot(core_slot: &Slot) -> ElementSlot {
        ElementSlot {
            name: core_slot.name.clone(),
            // TODO
            connections: Vec::new(),
        }
    }
}

pub struct ElementProvider {
    pub name: String,
    pub connections: Vec<ElementSlotRef>,
}

impl ElementProvider {
    fn from_core_provider(core_provider: &Provider) -> ElementProvider {
        ElementProvider {
            name: core_provider.name.clone(),
            // TODO
            connections: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ElementSlotRef {
    pub node: ElementRef,
    pub slot_index: usize,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ElementProviderRef {
    pub node: ElementRef,
    pub provider_index: usize,
}

impl Element {
    pub fn from_core_node(core_node_ref: &NodeRef) -> ElementRef {
        let core_node = core_node_ref.borrow_mut();
        let slots: Vec<_> = core_node
            .descriptor()
            .slots
            .iter()
            .map(|x| ElementSlot::from_core_slot(&x.borrow()))
            .collect();
        let providers: Vec<_> = core_node
            .descriptor()
            .providers
            .iter()
            .map(|x| ElementProvider::from_core_provider(&x.borrow()))
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
            "Flow node drop: {:?}, core refcount: {}",
            self,
            self.core_node.refc()
        );
    }
}

impl ElementSlotRef {
    pub fn new(node: &ElementRef, name: &str) -> ElementSlotRef {
        if let Some(index) = node.borrow().slots.iter().position(|x| x.name == name) {
            return ElementSlotRef {
                node: node.clone(),
                slot_index: index,
            };
        }
        panic!("Slot not found: '{}', node {:?}", name, node);
    }
}

impl ElementProviderRef {
    pub fn new(node_ref: &ElementRef, name: &str) -> ElementProviderRef {
        let node = node_ref.borrow();
        if let Some(index) = node.providers.iter().position(|x| x.name == name) {
            return ElementProviderRef {
                node: node_ref.clone(),
                provider_index: index,
            };
        }
        panic!("Provider not found: '{}', node {:?}", name, node);
    }
}
