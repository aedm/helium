use crate::flow::node::Node;
use crate::flow::provider::{Provider, ProviderValue};
use crate::flow::rf::{Rf, Weak};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

pub enum SlotType {
    Custom,
    Float32,
    Int64,
    Bool,
    Texture,
    Mesh,
    Render,
}

pub enum SlotDefault {
    None,
    Float32(f32),
    Int64(i64),
    Bool(bool),
}

pub enum SlotConnection {
    None,
    Single(Rf<Provider>),
    Multi(Vec<Rf<Provider>>),
}

pub trait SlotInner {
    fn can_connect(self: &Self, provider: &Provider) -> bool;
    fn get_type(self: &Self) -> SlotType;
}

pub struct Slot {
    pub owner: Weak<Node>,
    name: String,
    pub connection: SlotConnection,
    allow_multiple: bool,
    pub inner: Box<dyn SlotInner>,
    default: SlotDefault,
}

impl Slot {
    fn new(
        name: &str,
        allow_multiple: bool,
        inner: Box<dyn SlotInner>,
        default: SlotDefault,
    ) -> Slot {
        Slot {
            owner: Weak::new(),
            name: name.to_string(),
            connection: SlotConnection::None,
            allow_multiple,
            inner,
            default,
        }
    }

    pub fn set_default(&mut self, default: SlotDefault) {
        self.default = default;
    }
}

pub fn connect_slot(slot: &Rf<Slot>, provider: &Rf<Provider>) {
    let mut provider_mutref = provider.borrow_mut();
    let mut slot_mutref = slot.borrow_mut();
    if !slot_mutref.inner.can_connect(&provider_mutref) {
        panic!("Can't connect");
    }

    slot_mutref.connection = SlotConnection::Single(provider.clone());
    provider_mutref.connections.push(slot.clone());
}

pub struct FloatSlot {
    pub slot: Rf<Slot>,
}

impl FloatSlot {
    pub fn new(name: &str) -> FloatSlot {
        let inner = Box::new(FloatSlotInner {});
        let default = SlotDefault::Float32(10.0);
        FloatSlot {
            slot: Rf::new(Slot::new(name, false, inner, default)),
        }
    }

    pub fn get(self: &Self) -> f32 {
        let slot = &self.slot.borrow();
        if let SlotConnection::Single(p) = &slot.connection {
            if let ProviderValue::Float32(value) = &p.borrow().value {
                return *value;
            }
            panic!();
        }
        if let SlotDefault::Float32(value) = slot.default {
            return value;
        }
        panic!("No default for Float slot");
    }
}

struct FloatSlotInner {}

impl SlotInner for FloatSlotInner {
    fn can_connect(self: &Self, provider: &Provider) -> bool {
        if let ProviderValue::Float32(_) = provider.value {
            true
        } else {
            false
        }
    }

    fn get_type(self: &Self) -> SlotType {
        SlotType::Float32
    }
}
