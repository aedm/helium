use crate::render_graph::node_ref::{NodeRef, NodeWeak};
use crate::render_graph::provider::{Provider, ProviderValue};
use crate::render_graph::rcell::RCell;

pub struct NodeCoreProvider {
    pub provider: RCell<Provider>,
}

impl NodeCoreProvider {
    pub fn new(name: &str) -> NodeCoreProvider {
        NodeCoreProvider {
            provider: RCell::new(Provider::new(
                name,
                ProviderValue::Node(NodeWeak::new()),
            )),
        }
    }

    pub fn set(self: &mut Self, node: &NodeRef) {
        self.provider.borrow_mut().provider_value = ProviderValue::Node(node.downgrade());
    }
}
