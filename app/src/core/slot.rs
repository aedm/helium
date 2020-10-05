use crate::core::node::CoreNode;
use crate::core::provider::{CoreProvider, CoreProviderValue};
use crate::core::rf::{ACell, Weak};

pub enum SlotType {
    _Custom,
    Float32,
}

pub enum CoreSlotDefault {
    _None,
    Float32(f32),
}

pub enum CoreSlotConnection {
    None,
    Single(ACell<CoreProvider>),
    _Multi(Vec<ACell<CoreProvider>>),
}

pub trait CoreSlotInner {
    fn can_connect(self: &Self, provider: &CoreProvider) -> bool;
    fn get_type(self: &Self) -> SlotType;
}

pub struct CoreSlot {
    pub owner: Weak<CoreNode>,
    _name: String,
    pub connection: CoreSlotConnection,
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
            owner: Weak::new(),
            _name: name.to_string(),
            connection: CoreSlotConnection::None,
            _allow_multiple: allow_multiple,
            inner,
            default,
        }
    }

    pub fn set_default(&mut self, default: CoreSlotDefault) {
        self.default = default;
    }
}

pub fn connect_slot(slot: &ACell<CoreSlot>, provider: &ACell<CoreProvider>) {
    let mut provider_mutref = provider.borrow_mut();
    let mut slot_mutref = slot.borrow_mut();
    if !slot_mutref.inner.can_connect(&provider_mutref) {
        panic!("Can't connect");
    }
    slot_mutref.connection = CoreSlotConnection::Single(provider.clone());
    // provider_mutref.connections.push(slot.clone());
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
        if let CoreSlotConnection::Single(p) = &slot.connection {
            if let CoreProviderValue::Float32(value) = &p.borrow().provider_value {
                return *value;
            }
            panic!();
        }
        if let CoreSlotDefault::Float32(value) = slot.default {
            return value;
        }
        panic!("No default for Float slot");
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