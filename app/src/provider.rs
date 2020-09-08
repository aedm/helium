use crate::node::{ARef, Node, WeakRef, new_aref};
use crate::values::{Texture, Mesh};
use crate::slot::{FloatSlot, Slot};

pub trait RenderNode {
    fn render(self);
}

pub enum ProviderValue {
    None,
    Float32(f32),
    Int64(i64),
    Bool(bool),
    Texture(ARef<Texture>),
    Mesh(ARef<Mesh>),
    Render(ARef<dyn RenderNode>),
}

pub struct Provider {
    // pub owner: WeakRef<Node>,
    pub name: String,
    pub value: ProviderValue,
    pub connections: Vec<ARef<Slot>>,
}

impl Provider {
    fn new(name: &str, value: ProviderValue) -> Provider {
        Provider {
            name: name.to_string(),
            value,
            connections: vec![],
        }
    }
}

pub struct FloatProvider {
    pub provider: ARef<Provider>,
}

impl FloatProvider {
    pub fn new(name: &str) -> FloatProvider {
        FloatProvider {
            provider: new_aref(Provider::new(name, ProviderValue::Float32(0.0))),
        }
    }

    pub fn set(self: &mut Self, value: f32) {
        self.provider.borrow_mut().value = ProviderValue::Float32(value);
    }
}
