use crate::core::node::{CoreNode, CoreNodeRef};
use crate::flow::flow_node::{FlowNode, FlowNodeRef};
use crate::nodes::root_node::CoreRootNode;

pub struct _Stillaxis {
    core_root: CoreNodeRef,
    flow_root: FlowNodeRef,
}

impl _Stillaxis {
    pub fn _new() -> _Stillaxis {
        let core_root = CoreNode::new::<CoreRootNode>();
        let flow_root = FlowNode::from_core_node(&core_root);
        _Stillaxis {
            core_root,
            flow_root,
        }
    }
}
