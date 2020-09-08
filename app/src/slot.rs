use crate::provider::{Provider, ProviderValue};
use crate::node::{ARef, Node, WeakRef, new_aref};
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
    Single(ARef<Provider>),
    Multi(Vec<ARef<Provider>>),
}

pub trait SlotInner {
    fn can_connect(self: &Self, provider: &Provider) -> bool;
    fn get_type(self: &Self) -> SlotType;
}


pub struct Slot {
    // pub owner: WeakRef<Node>,
    name: String,
    pub connection: SlotConnection,
    allow_multiple: bool,
    pub inner: Box<dyn SlotInner>,
    default: SlotDefault,
}

impl Slot {
    fn new(name: &str, allow_multiple: bool, inner: Box<dyn SlotInner>) -> Slot {
        Slot {
            name: name.to_string(),
            connection: SlotConnection::None,
            allow_multiple,
            inner,
            default: SlotDefault::None,
        }
    }

    fn get_single(self: &Self) -> Option<ARef<Provider>> {
        if let SlotConnection::Single(x) = &self.connection {
            return Some(x.clone());
        }
        None
    }
}

pub fn connect_slot(slot: &mut ARef<Slot>, provider: &mut ARef<Provider>) {
    let mut provider_mutref = (provider as &RefCell<Provider>).borrow_mut();
    let mut slot_mutref = (slot as &RefCell<Slot>).borrow_mut();
    if !slot_mutref.inner.can_connect(&provider_mutref) {
        panic!("Can't connect");
    }

    slot_mutref.connection = SlotConnection::Single(provider.clone());
    provider_mutref.connections.push(slot.clone());
}

pub struct FloatSlot {
    pub slot: ARef<Slot>,
}

impl FloatSlot {
    pub fn new(name: &str) -> FloatSlot {
        FloatSlot {
            slot: new_aref(Slot::new(name, false, Box::new(FloatSlotInner {}))),
        }
    }

    pub fn get(self: &Self) -> f32 {
        let x = (&self.slot as &RefCell<Slot>).borrow();
        if let SlotConnection::Single(p) = &x.connection {
            let v = (&p as &RefCell<Provider>).borrow();
            if let ProviderValue::Float32(x) = v.value {
                return x;
            }
            panic!();
        }
        if let SlotDefault::Float32(x) = x.default {
            return x;
        }
        panic!("No default for Float slot");
    }
}

struct FloatSlotInner {}

impl SlotInner for FloatSlotInner {
    fn can_connect(self: &Self, provider: &Provider) -> bool {
        if let ProviderValue::Float32(_) = provider.value { true } else { false }
    }

    fn get_type(self: &Self) -> SlotType {
        SlotType::Float32
    }
}


