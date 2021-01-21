use crate::render_graph::mutation::{Mutation, SetSlotDefaultValueParams};
use crate::render_graph::slot::SlotDefault;
use crate::dom::dom::Dom;
use crate::dom::element::{DomElementRef, DomSlotRef};
use crate::dom::transaction::{TransactionStep, TransactionStepResult};

pub struct TransactionSetSlotValue {
    pub slot: DomSlotRef,
    pub value: SlotDefault,
}

impl TransactionSetSlotValue {
    pub fn new(
        element: &DomElementRef,
        slot_name: &str,
        value: SlotDefault,
    ) -> Box<TransactionSetSlotValue> {
        Box::new(TransactionSetSlotValue {
            slot: DomSlotRef::new(element, slot_name),
            value,
        })
    }
}

impl TransactionStep for TransactionSetSlotValue {
    fn run(&self, _dom: &mut Dom) -> TransactionStepResult {
        let core_mutation = Mutation::SetSlotDefaultValue(SetSlotDefaultValueParams {
            node: self.slot.element.borrow().core_node.clone(),
            slot_index: self.slot.slot_index,
            value: self.value,
        });
        TransactionStepResult {
            changed_slots: vec![],
            graph_mutations: vec![core_mutation],
        }
    }
}
