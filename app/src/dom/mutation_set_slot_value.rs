use crate::render_graph::graph_mutation::{GraphMutation, SetSlotDefaultValueParams};
use crate::render_graph::slot::SlotDefault;
use crate::dom::dom::Dom;
use crate::dom::dom_element::{DomElementRef, DomSlotRef};
use crate::dom::mutation::{DomMutationStep, DomMutationStepResult};

pub struct SetSlotValueDomMutation {
    pub node_slot: DomSlotRef,
    pub value: SlotDefault,
}

impl SetSlotValueDomMutation {
    pub fn _new(
        slot_node: &DomElementRef,
        slot_name: &str,
        value: SlotDefault,
    ) -> Box<SetSlotValueDomMutation> {
        Box::new(SetSlotValueDomMutation {
            node_slot: DomSlotRef::new(slot_node, slot_name),
            value,
        })
    }
}

impl DomMutationStep for SetSlotValueDomMutation {
    fn run(&self, _dom: &mut Dom) -> DomMutationStepResult {
        let core_mutation = GraphMutation::SetSlotDefaultValue(SetSlotDefaultValueParams {
            node: self.node_slot.node.borrow().core_node.clone(),
            slot_index: self.node_slot.slot_index,
            value: self.value,
        });
        DomMutationStepResult {
            changed_slots: vec![],
            core_mutations: vec![core_mutation],
        }
    }
}
