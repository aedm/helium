use crate::flow::dom::FlowDom;
use crate::flow::flow_node::FlowNodeRef;
use crate::flow::mutation::{FlowMutationStep, FlowMutationStepResult};

pub struct RemoveNodeFlowMutation {
    pub new_node: FlowNodeRef,
}

impl RemoveNodeFlowMutation {
    pub fn new(node: &FlowNodeRef) -> Box<RemoveNodeFlowMutation> {
        Box::new(RemoveNodeFlowMutation {
            new_node: node.clone(),
        })
    }
}

impl FlowMutationStep for RemoveNodeFlowMutation {
    fn run(&self, dom: &mut FlowDom) -> FlowMutationStepResult {
        dom.remove_flow_node(&self.new_node);
        FlowMutationStepResult {
            changed_slots: vec![],
            core_mutations: vec![],
        }
    }
}
