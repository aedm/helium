use super::dom::Dom;
use super::dom_element::DomSlotRef;
use super::mutation::{DomMutationStep, DomMutationStepResult};
use crate::dom::dom_element::DomProviderRef;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct SetSlotConnectionsDomMutation {
    pub node_slot: DomSlotRef,
    pub connections: Vec<DomProviderRef>,
}

impl SetSlotConnectionsDomMutation {
    pub fn new(
        node_slot: DomSlotRef,
        connections: Vec<DomProviderRef>,
    ) -> Box<SetSlotConnectionsDomMutation> {
        Box::new(SetSlotConnectionsDomMutation {
            node_slot,
            connections,
        })
    }
}

impl DomMutationStep for SetSlotConnectionsDomMutation {
    fn run(&self, _dom: &mut Dom) -> DomMutationStepResult {
        // Change dom DOM
        let mut node = self.node_slot.node.borrow_mut();
        let slot = &mut node.slots[self.node_slot.slot_index];

        let providers_to_remove = HashSet::from_iter(slot.connections.iter());
        let providers_to_add = HashSet::from_iter(self.connections.iter());
        let intersection: HashSet<_> = providers_to_remove
            .intersection(&providers_to_add)
            .map(|x| *x)
            .collect();

        providers_to_remove
            .difference(&intersection)
            .for_each(|x| _dom.remove_slot_from_provider(*x, &self.node_slot));
        providers_to_add
            .difference(&intersection)
            .for_each(|x| _dom.add_slot_to_provider(*x, &self.node_slot));

        // TODO: Use mem::swap here?
        slot.connections = self.connections.to_vec();

        // Generate render_graph mutation
        DomMutationStepResult {
            changed_slots: vec![self.node_slot.clone()],
            core_mutations: vec![],
        }
    }
}
