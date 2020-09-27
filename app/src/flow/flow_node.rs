use crate::core::node::NodeId;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

static NODE_ID_GENERATOR: AtomicU64 = AtomicU64::new(1);

enum ProviderType {
    None,
    Float32,
}

pub struct FlowNode {
    pub id: NodeId,
    pub key: String,
}

pub type FlowNodeRef = Rc<RefCell<FlowNode>>;

struct FlowSlot {
    id: String,
}

struct FlowProvider {
    id: String,
}

impl FlowNode {
    fn new() -> FlowNode {
        FlowNode {
            id: NODE_ID_GENERATOR.fetch_add(1, Ordering::Relaxed),
            key: "".into(),
        }
    }
}
