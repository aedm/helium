use crate::core::node::CoreNode;
use crate::core::provider::{CoreProvider, CoreProviderValue};
use crate::core::acell::{ACell, AWeak};

pub enum SlotType {
    _Custom,
    Float32,
}

pub enum CoreSlotDefault {
    _None,
    Float32(f32),
}

pub trait CoreSlotInner {
    fn can_connect(self: &Self, provider: &CoreProvider) -> bool;
    fn get_type(self: &Self) -> SlotType;
}

pub struct CoreSlot {
    pub owner: AWeak<CoreNode>,
    _name: String,
    pub connection: Vec<ACell<CoreProvider>>,
    _allow_multiple: bool,
    pub inner: Box<dyn CoreSlotInner>,
    default: CoreSlotDefault,
}

impl CoreSlot {
    fn new(
        name: &str,
        allow_multiple: bool,
        inner: Box<dyn CoreSlotInner>,
        default: CoreSlotDefault,
    ) -> CoreSlot {
        CoreSlot {
            owner: AWeak::new(),
            _name: name.to_string(),
            connection: vec![],
            _allow_multiple: allow_multiple,
            inner,
            default,
        }
    }

    pub fn set_default(&mut self, default: CoreSlotDefault) {
        self.default = default;
    }

    pub fn get_single_provider(&self) -> Option<&ACell<CoreProvider>> {
        match self.connection.len() {
            0 => None,
            1 => Some(&self.connection[0]),
            _ => panic!("'get_single_provider' called, multiple providers connected.")
        }
    }
}

pub struct FloatCoreSlot {
    pub slot: ACell<CoreSlot>,
}

impl FloatCoreSlot {
    pub fn new(name: &str) -> FloatCoreSlot {
        let inner = Box::new(FloatCoreSlotInner {});
        let default = CoreSlotDefault::Float32(10.0);
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
