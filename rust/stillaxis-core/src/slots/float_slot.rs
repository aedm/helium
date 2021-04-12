use crate::provider::{Provider, ProviderValue};
use crate::rcell::RCell;
use crate::slot::{Slot, SlotDefaultValue, SlotInner, SlotType};

pub struct FloatSlot {
    pub slot: RCell<Slot>,
}

impl FloatSlot {
    pub fn new(name: &str) -> FloatSlot {
        let inner = Box::new(FloatSlotInner {});
        let default = SlotDefaultValue::Float32(0.0);
        FloatSlot {
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
        if let SlotDefaultValue::Float32(value) = slot.default_value {
            return value;
        }
        panic!("Float slot's default value is not a float.");
    }
}

struct FloatSlotInner {}

impl SlotInner for FloatSlotInner {
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
