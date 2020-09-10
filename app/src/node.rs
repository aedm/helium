use std::sync::{Arc, Weak};
use std::cell::RefCell;
use std::borrow::Borrow;
use std::ops::Deref;
use crate::provider::{Provider, FloatProvider};
use crate::slot::{Slot, FloatSlot, SlotConnection};
use crate::rf::Rf;

pub struct Node {
    pub slots: Vec<Rf<Slot>>,
    pub providers: Vec<Rf<Provider>>,
    inner: Box<dyn NodeInner>,
}

pub trait NodeInner {
    fn new() -> Self where Self: Sized;
    fn get_slots(self: &Self) -> Vec<Rf<Slot>>;
    fn get_providers(self: &Self) -> Vec<Rf<Provider>>;
    fn run(self: &mut Self);
}

impl Node {
    pub fn new<T: 'static + NodeInner>() -> Node {
        let inner = Box::new(T::new());
        Node {
            slots: inner.get_slots(),
            providers: inner.get_providers(),
            inner,
        }
    }

    pub fn run(&mut self) {
        self.inner.run();
    }
}

pub struct FloatNode {
    pub a: FloatSlot,
    pub out: FloatProvider,
}

impl NodeInner for FloatNode {
    fn new() -> FloatNode {
        FloatNode {
            a: FloatSlot::new("A"),
            out: FloatProvider::new("Value"),
        }
    }

    fn get_slots(self: &Self) -> Vec<Rf<Slot>> {
        vec![self.a.slot.clone()]
    }

    fn get_providers(self: &Self) -> Vec<Rf<Provider>> {
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

impl NodeInner for Adder {
    fn new() -> Adder {
        Adder {
            a: FloatSlot::new("A"),
            b: FloatSlot::new("B"),
            sum: FloatProvider::new("Sum"),
        }
    }

    fn get_slots(self: &Self) -> Vec<Rf<Slot>> {
        vec![
            self.a.slot.clone(),
            self.b.slot.clone()
        ]
    }

    fn get_providers(self: &Self) -> Vec<Rf<Provider>> {
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