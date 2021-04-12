use crate::node_ref::NodeRef;
use crate::provider::{Provider, ProviderValue};
use crate::rcell::RCell;
use crate::slot::{Slot, SlotDefaultValue, SlotInner, SlotType};

pub struct NodeSlot {
    pub slot: RCell<Slot>,
}

impl NodeSlot {
    pub fn new(name: &str) -> NodeSlot {
        let inner = Box::new(NodeSlotInner {});
        let default = SlotDefaultValue::None;
        NodeSlot {
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

struct NodeSlotInner {}

impl SlotInner for NodeSlotInner {
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
