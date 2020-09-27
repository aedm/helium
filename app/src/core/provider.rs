use crate::core::node::CoreNode;
use crate::core::rf::{Rf, Weak};
use crate::core::slot::CoreSlot;

#[derive(Debug)]
pub enum CoreProviderValue {
    _None,
    Float32(f32),
}

pub struct CoreProvider {
    pub owner: Weak<CoreNode>,
    pub name: String,
    pub provider_value: CoreProviderValue,
    // pub connections: Vec<Rf<CoreSlot>>,
}

impl CoreProvider {
    fn new(name: &str, value: CoreProviderValue) -> CoreProvider {
        CoreProvider {
            owner: Weak::new(),
            name: name.to_string(),
            provider_value: value,
            // connections: vec![],
        }
    }
}

pub struct FloatCoreProvider {
    pub provider: Rf<CoreProvider>,
}

impl FloatCoreProvider {
    pub fn new(name: &str) -> FloatCoreProvider {
        FloatCoreProvider {
            provider: Rf::new(CoreProvider::new(name, CoreProviderValue::Float32(0.0))),
        }
    }

    pub fn set(self: &mut Self, value: f32) {
        self.provider.borrow_mut().provider_value = CoreProviderValue::Float32(value);
    }
}
