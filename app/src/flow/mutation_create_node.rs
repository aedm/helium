use crate::flow::flow_node::FlowNodeRef;
use crate::flow::mutation::{FlowMutationStep, FlowMutationStepResult};
use crate::flow::dom::Dom;

pub struct CreateNodeFlowMutation {
    pub new_node: FlowNodeRef,
}

impl FlowMutationStep for CreateNodeFlowMutation {
    fn run(&self, dom: &mut Dom) -> FlowMutationStepResult {
        dom.add_flow_node(&self.new_node);
        FlowMutationStepResult {
            changed_slots: vec![],
        }
    }
}