use crate::core::node::{CoreNode, CoreNodeRef};
use crate::flow::flow_node::{FlowNode, FlowNodeRef};
use crate::nodes::root_node::CoreRootNode;
use crate::flow::dom::FlowDom;
use crate::flow::mutation::FlowMutation;
use std::thread;
use crate::core::core_dom::CoreDom;

pub struct Stillaxis {
    core_dom: CoreDom,
    flow_dom: FlowDom,
}

impl Stillaxis {
    pub fn _new() -> Stillaxis {
        let core_dom = CoreDom::new();
        let mut flow_dom = FlowDom::new(&core_dom);

        Stillaxis {
            core_dom,
            flow_dom,
        }
    }

    pub fn run_mutation(&mut self, flow_mutation: &mut FlowMutation) {
        let core_mutation = flow_mutation.run(&mut self.flow_dom);

    }
}
