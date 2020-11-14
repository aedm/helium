use crate::core::acell::ACell;
use crate::core::provider::{CoreProvider, CoreProviderValue};

pub struct FloatCoreProvider {
    pub provider: ACell<CoreProvider>,
}

impl FloatCoreProvider {
    pub fn new(name: &str) -> FloatCoreProvider {
        FloatCoreProvider {
            provider: ACell::new(CoreProvider::new(name, CoreProviderValue::Float32(0.0))),
        }
    }

    pub fn set(self: &mut Self, value: f32) {
        self.provider.borrow_mut().provider_value = CoreProviderValue::Float32(value);
    }
}