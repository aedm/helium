use crate::dom::dom::Dom;
use crate::dom::dom_element::DomElementRef;
use crate::dom::mutation::{DomMutationStep, DomMutationStepResult};

pub struct RemoveNodeDomMutation {
    pub new_node: DomElementRef,
}

impl RemoveNodeDomMutation {
    pub fn new(node: &DomElementRef) -> Box<RemoveNodeDomMutation> {
        Box::new(RemoveNodeDomMutation {
            new_node: node.clone(),
        })
    }
}

impl DomMutationStep for RemoveNodeDomMutation {
    fn run(&self, dom: &mut Dom) -> DomMutationStepResult {
        dom.remove_flow_node(&self.new_node);
        DomMutationStepResult {
            changed_slots: vec![],
            core_mutations: vec![],
        }
    }
}
