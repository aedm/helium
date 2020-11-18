use crate::core::provider::{CoreProvider, CoreProviderValue};
use crate::core::rcell::RCell;

pub struct FloatCoreProvider {
    pub provider: RCell<CoreProvider>,
}

impl FloatCoreProvider {
    pub fn new(name: &str) -> FloatCoreProvider {
        FloatCoreProvider {
            provider: RCell::new(CoreProvider::new(name, CoreProviderValue::Float32(0.0))),
        }
    }

    pub fn set(self: &mut Self, value: f32) {
        self.provider.borrow_mut().provider_value = CoreProviderValue::Float32(value);
    }
}
