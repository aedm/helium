use crate::core::acell::ACell;
use crate::core::slot::{CoreSlot, CoreSlotDefault, CoreSlotInner, SlotType};
use crate::core::provider::{CoreProviderValue, CoreProvider};

pub struct FloatCoreSlot {
    pub slot: ACell<CoreSlot>,
}

impl FloatCoreSlot {
    pub fn new(name: &str) -> FloatCoreSlot {
        let inner = Box::new(FloatCoreSlotInner {});
        let default = CoreSlotDefault::Float32(0.0);
        FloatCoreSlot {
            slot: ACell::new(CoreSlot::new(name, false, inner, default)),
        }
    }

    pub fn get(self: &Self) -> f32 {
        let slot = &self.slot.borrow();
        if let Some(provider) = slot.get_single_provider() {
            if let CoreProviderValue::Float32(value) = &provider.borrow().provider_value {
                return *value;
            }
            panic!("Float slot connected to a non-float provider");
        }
        if let CoreSlotDefault::Float32(value) = slot.default {
            return value;
        }
        panic!("Float slot's default value is not a float.");
    }
}

struct FloatCoreSlotInner {}

impl CoreSlotInner for FloatCoreSlotInner {
    fn can_connect(self: &Self, provider: &CoreProvider) -> bool {
        match provider.provider_value {
            CoreProviderValue::Float32(_) => true,
            _ => false,
        }
    }

    fn get_type(self: &Self) -> SlotType {
        SlotType::Float32
    }
}