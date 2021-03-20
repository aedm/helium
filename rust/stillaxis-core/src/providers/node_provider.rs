use crate::node_ref::{CoreNodeRef, CoreNodeWeak};
use crate::provider::{CoreProvider, CoreProviderValue};
use crate::rcell::RCell;

pub struct NodeCoreProvider {
    pub provider: RCell<CoreProvider>,
}

impl NodeCoreProvider {
    pub fn new(name: &str) -> NodeCoreProvider {
        NodeCoreProvider {
            provider: RCell::new(CoreProvider::new(
                name,
                CoreProviderValue::Node(CoreNodeWeak::new()),
            )),
        }
    }

    pub fn set(self: &mut Self, node: &CoreNodeRef) {
        self.provider.borrow_mut().provider_value = CoreProviderValue::Node(node.downgrade());
    }
}
