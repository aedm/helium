use std::sync::{Arc, Weak};
use std::cell::RefCell;
use std::borrow::Borrow;
use std::ops::Deref;
use crate::flow::provider::{Provider, FloatProvider};
use crate::flow::slot::{Slot, FloatSlot, SlotConnection};
use crate::flow::rf::Rf;

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
