use crate::render_graph::node_ref::NodeRef;
use crate::render_graph::provider::{Provider, ProviderValue};
use crate::render_graph::rcell::RCell;
use crate::render_graph::slot::{Slot, SlotDefault, SlotInner, SlotType};

pub struct NodeCoreSlot {
    pub slot: RCell<Slot>,
}

impl NodeCoreSlot {
    pub fn new(name: &str) -> NodeCoreSlot {
        let inner = Box::new(NodeCoreSlotInner {});
        let default = SlotDefault::None;
        NodeCoreSlot {
            slot: RCell::new(Slot::new(name, false, inner, default)),
        }
    }

    pub fn _get(self: &Self) -> Option<NodeRef> {
        let slot = &self.slot.borrow();
        if let Some(provider) = slot.get_single_provider() {
            if let ProviderValue::Node(value) = &provider.borrow().provider_value {
                let node_ref = value.upgrade();
                if let None = node_ref {
                    panic!("Node slot connected to a node provider without owner");
                }
                return node_ref;
            }
            panic!("Node slot connected to a non-node provider");
        }
        None
    }
}

struct NodeCoreSlotInner {}

impl SlotInner for NodeCoreSlotInner {
    fn can_connect(self: &Self, provider: &Provider) -> bool {
        match provider.provider_value {
            ProviderValue::Node(_) => true,
            _ => false,
        }
    }

    fn get_type(self: &Self) -> SlotType {
        SlotType::Node
    }
}
