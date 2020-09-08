use std::sync::{Arc, Weak};
use std::cell::RefCell;
use std::borrow::Borrow;
use std::ops::Deref;
use crate::provider::{Provider, FloatProvider};
use crate::slot::{Slot, FloatSlot, SlotConnection};

pub type ARef<T> = Arc<RefCell<T>>;
pub type WeakRef<T> = Weak<RefCell<T>>;

pub fn new_aref<T>(t: T) -> ARef<T> {
    Arc::new(RefCell::new(t))
}

pub struct Node {
    pub slots: Vec<ARef<Slot>>,
    pub providers: Vec<ARef<Provider>>,
    pub inner: Box<dyn NodeInner>,
}

pub trait NodeInner {
    fn get_slots(self: &Self) -> Vec<ARef<Slot>>;
    fn get_providers(self: &Self) -> Vec<ARef<Provider>>;
    fn run(self: &mut Self);
}

impl Node {
    pub fn new(inner: Box<dyn NodeInner>) -> Node {
        Node {
            slots: inner.get_slots(),
            providers: inner.get_providers(),
            inner,
        }
    }
}

pub struct FloatNode {
    pub a: FloatSlot,
    pub out: FloatProvider,
}

impl FloatNode {
    pub fn new() -> FloatNode {
        FloatNode {
            a: FloatSlot::new("A"),
            out: FloatProvider::new("Value"),
        }
    }
}

impl NodeInner for FloatNode {
    fn get_slots(self: &Self) -> Vec<ARef<Slot>> {
        vec![self.a.slot.clone()]
    }

    fn get_providers(self: &Self) -> Vec<ARef<Provider>> {
        vec![self.out.provider.clone()]
    }

    fn run(self: &mut Self) {
        self.out.set(self.a.get());
    }
}

pub struct Adder {
    pub a: FloatSlot,
    pub b: FloatSlot,
    pub sum: FloatProvider,
}

impl Adder {
    pub fn new() -> Adder {
        Adder {
            a: FloatSlot::new("A"),
            b: FloatSlot::new("B"),
            sum: FloatProvider::new("Sum"),
        }
    }
}

impl NodeInner for Adder {
    fn get_slots(self: &Self) -> Vec<ARef<Slot>> {
        vec![
            self.a.slot.clone(),
            self.b.slot.clone()
        ]
    }

    fn get_providers(self: &Self) -> Vec<ARef<Provider>> {
        vec![
            self.sum.provider.clone(),
        ]
    }

    fn run(self: &mut Self) {
        let result = self.a.get() + self.b.get();
        self.sum.set(result);
        println!("Result: {}", result);
    }
}