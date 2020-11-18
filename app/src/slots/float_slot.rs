use crate::core::provider::{CoreProvider, CoreProviderValue};
use crate::core::rcell::RCell;
use crate::core::slot::{CoreSlot, CoreSlotDefault, CoreSlotInner, SlotType};

pub struct FloatCoreSlot {
    pub slot: RCell<CoreSlot>,
}

impl FloatCoreSlot {
    pub fn new(name: &str) -> FloatCoreSlot {
        let inner = Box::new(FloatCoreSlotInner {});
        let default = CoreSlotDefault::Float32(0.0);
        FloatCoreSlot {
            slot: RCell::new(CoreSlot::new(name, false, inner, default)),
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
