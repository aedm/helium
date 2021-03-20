use crate::flow::dom::FlowDom;
use crate::flow::flow_node::{FlowNodeRef, FlowSlotIndex};
use crate::flow::mutation::{FlowMutationStep, FlowMutationStepResult};
use stillaxis_core::core_mutation::{CoreMutation, SetSlotDefaultValueParams};
use stillaxis_core::slot::CoreSlotDefault;

pub struct SetSlotValueFlowMutation {
    pub node_slot: FlowSlotIndex,
    pub value: CoreSlotDefault,
}

impl SetSlotValueFlowMutation {
    pub fn _new(
        slot_node: &FlowNodeRef,
        slot_name: &str,
        value: CoreSlotDefault,
    ) -> Box<SetSlotValueFlowMutation> {
        Box::new(SetSlotValueFlowMutation {
            node_slot: FlowSlotIndex::new(slot_node, slot_name),
            value,
        })
    }
}

impl FlowMutationStep for SetSlotValueFlowMutation {
    fn run(&self, _dom: &mut FlowDom) -> FlowMutationStepResult {
        let core_mutation = CoreMutation::SetSlotDefaultValue(SetSlotDefaultValueParams {
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
