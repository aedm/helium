use crate::node_ref::NodeWeakRef;

#[derive(Debug, Clone, PartialEq)]
pub enum ProviderValue {
    Node(NodeWeakRef),
    Float32(f32),
}

impl Eq for ProviderValue {}

pub struct Provider {
    pub name: String,
    pub provider_value: ProviderValue,
}

impl Provider {
    pub fn new(name: &str, value: ProviderValue) -> Provider {
        Provider {
            name: name.to_string(),
            provider_value: value,
        }
    }
}
