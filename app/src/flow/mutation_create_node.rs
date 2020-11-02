use crate::flow::dom::Dom;
use crate::flow::flow_node::FlowNodeRef;
use crate::flow::mutation::{FlowMutationStep, FlowMutationStepResult};

pub struct CreateNodeFlowMutation {
    pub new_node: FlowNodeRef,
}

impl CreateNodeFlowMutation {
    pub fn new(node: &FlowNodeRef) -> Box<CreateNodeFlowMutation> {
        Box::new(CreateNodeFlowMutation {
            new_node: node.clone(),
        })
    }
}

impl FlowMutationStep for CreateNodeFlowMutation {
    fn run(&self, dom: &mut Dom) -> FlowMutationStepResult {
        dom.add_flow_node(&self.new_node);
        FlowMutationStepResult {
            changed_slots: vec![],
        }
    }
}
