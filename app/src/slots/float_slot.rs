use crate::render_graph::provider::{Provider, ProviderValue};
use crate::render_graph::rcell::RCell;
use crate::render_graph::slot::{Slot, SlotDefault, SlotInner, SlotType};

pub struct FloatCoreSlot {
    pub slot: RCell<Slot>,
}

impl FloatCoreSlot {
    pub fn new(name: &str) -> FloatCoreSlot {
        let inner = Box::new(FloatCoreSlotInner {});
        let default = SlotDefault::Float32(0.0);
        FloatCoreSlot {
            slot: RCell::new(Slot::new(name, false, inner, default)),
        }
    }

    pub fn get(self: &Self) -> f32 {
        let slot = &self.slot.borrow();
        if let Some(provider) = slot.get_single_provider() {
            if let ProviderValue::Float32(value) = &provider.borrow().provider_value {
                return *value;
            }
            panic!("Float slot connected to a non-float provider");
        }
        if let SlotDefault::Float32(value) = slot.default {
            return value;
        }
        panic!("Float slot's default value is not a float.");
    }
}

struct FloatCoreSlotInner {}

impl SlotInner for FloatCoreSlotInner {
    fn can_connect(self: &Self, provider: &Provider) -> bool {
        match provider.provider_value {
            ProviderValue::Float32(_) => true,
            _ => false,
        }
    }

    fn get_type(self: &Self) -> SlotType {
        SlotType::Float32
    }
}
