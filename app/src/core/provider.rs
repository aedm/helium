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
    pub fn new(name: &str, value: CoreProviderValue) -> CoreProvider {
        CoreProvider {
            name: name.to_string(),
            provider_value: value,
        }
    }
}


