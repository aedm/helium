use crate::flow::node::{Node};
use crate::flow::slot::{FloatSlot, Slot};
use crate::flow::rf::{Rf, Weak};

pub trait RenderNode {
    fn render(self);
}

pub struct Texture {}
pub struct Mesh {}

#[derive(Debug)]
pub enum ProviderValue {
    None,
    Float32(f32),
    Int64(i64),
    Bool(bool),
    Texture(Rf<Texture>),
    Mesh(Rf<Mesh>),
    Render(Rf<dyn RenderNode>),
}

pub struct Provider {
    pub owner: Weak<Node>,
    pub name: String,
    pub value: ProviderValue,
    pub connections: Vec<Rf<Slot>>,
}

impl Provider {
    fn new(name: &str, value: ProviderValue) -> Provider {
        Provider {
            owner: Weak::new(),
            name: name.to_string(),
            value,
            connections: vec![],
        }
    }
}

pub struct FloatProvider {
    pub provider: Rf<Provider>,
}

impl FloatProvider {
    pub fn new(name: &str) -> FloatProvider {
        FloatProvider {
            provider: Rf::new(Provider::new(name, ProviderValue::Float32(0.0))),
        }
    }

    pub fn set(self: &mut Self, value: f32) {
        self.provider.borrow_mut().value = ProviderValue::Float32(value);
    }
}
