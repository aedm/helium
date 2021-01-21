use crate::dom::dom::Dom;
use crate::dom::dom_element::DomElementRef;
use crate::dom::mutation::{DomMutationStep, DomMutationStepResult};

pub struct CreateNodeDomMutation {
    pub new_node: DomElementRef,
}

impl CreateNodeDomMutation {
    pub fn new(node: &DomElementRef) -> Box<CreateNodeDomMutation> {
        Box::new(CreateNodeDomMutation {
            new_node: node.clone(),
        })
    }
}

impl DomMutationStep for CreateNodeDomMutation {
    fn run(&self, dom: &mut Dom) -> DomMutationStepResult {
        dom.add_flow_node(&self.new_node);
        DomMutationStepResult {
            changed_slots: vec![],
            core_mutations: vec![],
        }
    }
}
