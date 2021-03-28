use crate::dom::document::Document;
use crate::dom::flow_node::{ElementRef, ElementSlotRef};
use crate::dom::mutation::{FlowMutationStep, FlowMutationStepResult};
use stillaxis_core::mutation::{Mutation, SetSlotDefaultValueParams};
use stillaxis_core::slot::SlotDefaultValue;

pub struct SetSlotValueFlowMutation {
    pub node_slot: ElementSlotRef,
    pub value: SlotDefaultValue,
}

impl SetSlotValueFlowMutation {
    pub fn _new(
        slot_node: &ElementRef,
        slot_name: &str,
        value: SlotDefaultValue,
    ) -> Box<SetSlotValueFlowMutation> {
        Box::new(SetSlotValueFlowMutation {
            node_slot: ElementSlotRef::new(slot_node, slot_name),
            value,
        })
    }
}

impl FlowMutationStep for SetSlotValueFlowMutation {
    fn run(&self, _dom: &mut Document) -> FlowMutationStepResult {
        let core_mutation = Mutation::SetSlotDefaultValue(SetSlotDefaultValueParams {
            node: self.node_slot.node.borrow().core_node.clone(),
            slot_index: self.node_slot.slot_index,
            value: self.value,
        });
        FlowMutationStepResult {
            changed_slots: vec![],
            core_mutations: vec![core_mutation],
        }
    }
}
