use crate::core::slot::CoreSlotDefault;
use crate::flow::flow_node::{FlowSlotIndex, FlowNodeRef};
use crate::flow::mutation::{FlowMutationStep, FlowMutationStepResult};
use crate::flow::dom::FlowDom;
use crate::core::core_mutation::SetSlotDefaultValueCoreMutation;

pub struct SetSlotValueFlowMutation {
    pub node_slot: FlowSlotIndex,
    pub value: CoreSlotDefault,
}

impl SetSlotValueFlowMutation {
    pub fn new(slot_node: &FlowNodeRef,
               slot_index: usize,
               value: CoreSlotDefault) -> Box<SetSlotValueFlowMutation> {
        Box::new(SetSlotValueFlowMutation {
            node_slot: FlowSlotIndex {
                node: slot_node.clone(),
                slot_index,
            },
            value,
        })
    }
}

impl FlowMutationStep for SetSlotValueFlowMutation {
    fn run(&self, dom: &mut FlowDom) -> FlowMutationStepResult {
        let core_mutation = SetSlotDefaultValueCoreMutation {
            node: self.node_slot.node.borrow().core_node.clone(),
            slot_index: self.node_slot.slot_index,
            value: self.value,
        };
        FlowMutationStepResult {
            changed_slots: vec![],
            core_mutations: vec![Box::new(core_mutation)],
        }
    }
}

