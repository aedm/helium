use crate::dom::document::Document;
use crate::dom::flow_node::ElementRef;
use crate::dom::mutation::{FlowMutationStep, FlowMutationStepResult};

pub struct CreateNodeFlowMutation {
    pub new_node: ElementRef,
}

impl CreateNodeFlowMutation {
    pub fn new(node: &ElementRef) -> Box<CreateNodeFlowMutation> {
        Box::new(CreateNodeFlowMutation {
            new_node: node.clone(),
        })
    }
}

impl FlowMutationStep for CreateNodeFlowMutation {
    fn run(&self, dom: &mut Document) -> FlowMutationStepResult {
        dom.add_component(&self.new_node);
        FlowMutationStepResult {
            changed_slots: vec![],
            core_mutations: vec![],
        }
    }
}
