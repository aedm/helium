use crate::provider::{Provider, ProviderValue};
use crate::rcell::RCell;

pub struct FloatProvider {
    pub provider: RCell<Provider>,
}

impl FloatProvider {
    pub fn new(name: &str) -> FloatProvider {
        FloatProvider {
            provider: RCell::new(Provider::new(name, ProviderValue::Float32(0.0))),
        }
    }

    pub fn set(self: &mut Self, value: f32) {
        self.provider.borrow_mut().provider_value = ProviderValue::Float32(value);
    }
}
