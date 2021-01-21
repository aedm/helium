use crate::render_graph::provider::{Provider, ProviderValue};
use crate::render_graph::rcell::RCell;

pub struct FloatCoreProvider {
    pub provider: RCell<Provider>,
}

impl FloatCoreProvider {
    pub fn new(name: &str) -> FloatCoreProvider {
        FloatCoreProvider {
            provider: RCell::new(Provider::new(name, ProviderValue::Float32(0.0))),
        }
    }

    pub fn set(self: &mut Self, value: f32) {
        self.provider.borrow_mut().provider_value = ProviderValue::Float32(value);
    }
}
