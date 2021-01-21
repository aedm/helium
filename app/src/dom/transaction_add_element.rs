use crate::dom::dom::Dom;
use crate::dom::element::DomElementRef;
use crate::dom::transaction::{TransactionStep, TransactionStepResult};

pub struct TransactionAddElement {
    new_element: DomElementRef,
}

impl TransactionAddElement {
    pub fn new(element: &DomElementRef) -> Box<TransactionAddElement> {
        Box::new(TransactionAddElement {
            new_element: element.clone(),
        })
    }
}

impl TransactionStep for TransactionAddElement {
    fn run(&self, dom: &mut Dom) -> TransactionStepResult {
        dom.add_flow_node(&self.new_element);
        TransactionStepResult {
            changed_slots: vec![],
            graph_mutations: vec![],
        }
    }
}
