use crate::dom::document::Document;
use crate::dom::flow_node::ElementRef;
use crate::dom::mutation::{FlowMutationStep, FlowMutationStepResult};

pub struct RemoveNodeFlowMutation {
    pub new_node: ElementRef,
}

impl RemoveNodeFlowMutation {
    pub fn new(node: &ElementRef) -> Box<RemoveNodeFlowMutation> {
        Box::new(RemoveNodeFlowMutation {
            new_node: node.clone(),
        })
    }
}

impl FlowMutationStep for RemoveNodeFlowMutation {
    fn run(&self, dom: &mut Document) -> FlowMutationStepResult {
        dom.remove_component(&self.new_node);
        FlowMutationStepResult {
            changed_slots: vec![],
            core_mutations: vec![],
        }
    }
}
