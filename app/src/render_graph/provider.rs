use crate::render_graph::node_ref::NodeWeak;

#[derive(Debug, Clone, PartialEq)]
pub enum ProviderValue {
    Node(NodeWeak),
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
