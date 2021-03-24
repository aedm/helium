use crate::node_ref::{NodeRef, NodeWeakRef};
use crate::provider::{Provider, ProviderValue};
use crate::rcell::RCell;

pub struct NodeProvider {
    pub provider: RCell<Provider>,
}

impl NodeProvider {
    pub fn new(name: &str) -> NodeProvider {
        NodeProvider {
            provider: RCell::new(Provider::new(
                name,
                ProviderValue::Node(NodeWeakRef::new()),
            )),
        }
    }

    pub fn set(self: &mut Self, node: &NodeRef) {
        self.provider.borrow_mut().provider_value = ProviderValue::Node(node.downgrade());
    }
}
