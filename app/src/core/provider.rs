use crate::core::acell::ACell;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoreProviderValue {
    _None,
    Float32(f32),
}

impl Eq for CoreProviderValue {}

pub struct CoreProvider {
    pub name: String,
    pub provider_value: CoreProviderValue,
}

impl CoreProvider {
    fn new(name: &str, value: CoreProviderValue) -> CoreProvider {
        CoreProvider {
            name: name.to_string(),
            provider_value: value,
        }
    }
}

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
