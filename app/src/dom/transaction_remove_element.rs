use crate::dom::dom::Dom;
use crate::dom::element::DomElementRef;
use crate::dom::transaction::{TransactionStep, TransactionStepResult};

pub struct TransactionRemoveElement {
    pub new_node: DomElementRef,
}

impl TransactionRemoveElement {
    pub fn new(node: &DomElementRef) -> Box<TransactionRemoveElement> {
        Box::new(TransactionRemoveElement {
            new_node: node.clone(),
        })
    }
}

impl TransactionStep for TransactionRemoveElement {
    fn run(&self, dom: &mut Dom) -> TransactionStepResult {
        dom.remove_flow_node(&self.new_node);
        TransactionStepResult {
            changed_slots: vec![],
            graph_mutations: vec![],
        }
    }
}
