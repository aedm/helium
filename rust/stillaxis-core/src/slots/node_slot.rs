use crate::node_ref::CoreNodeRef;
use crate::provider::{CoreProvider, CoreProviderValue};
use crate::rcell::RCell;
use crate::slot::{CoreSlot, CoreSlotDefault, CoreSlotInner, SlotType};

pub struct NodeCoreSlot {
    pub slot: RCell<CoreSlot>,
}

impl NodeCoreSlot {
    pub fn new(name: &str) -> NodeCoreSlot {
        let inner = Box::new(NodeCoreSlotInner {});
        let default = CoreSlotDefault::None;
        NodeCoreSlot {
            slot: RCell::new(CoreSlot::new(name, false, inner, default)),
        }
    }

    pub fn _get(self: &Self) -> Option<CoreNodeRef> {
        let slot = &self.slot.borrow();
        if let Some(provider) = slot.get_single_provider() {
            if let CoreProviderValue::Node(value) = &provider.borrow().provider_value {
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

impl CoreSlotInner for NodeCoreSlotInner {
    fn can_connect(self: &Self, provider: &CoreProvider) -> bool {
        match provider.provider_value {
            CoreProviderValue::Node(_) => true,
            _ => false,
        }
    }

    fn get_type(self: &Self) -> SlotType {
        SlotType::Node
    }
}
