use super::dom::Dom;
use super::element::DomSlotRef;
use super::transaction::{TransactionStep, TransactionStepResult};
use crate::dom::element::DomProviderRef;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct TransactionSetSlotConnections {
    pub slot: DomSlotRef,
    pub connections: Vec<DomProviderRef>,
}

impl TransactionSetSlotConnections {
    pub fn new(
        slot: DomSlotRef,
        connections: Vec<DomProviderRef>,
    ) -> Box<TransactionSetSlotConnections> {
        Box::new(TransactionSetSlotConnections {
            slot,
            connections,
        })
    }
}

impl TransactionStep for TransactionSetSlotConnections {
    fn run(&self, _dom: &mut Dom) -> TransactionStepResult {
        // Change dom DOM
        let mut node = self.slot.element.borrow_mut();
        let slot = &mut node.slots[self.slot.slot_index];

        let providers_to_remove = HashSet::from_iter(slot.connections.iter());
        let providers_to_add = HashSet::from_iter(self.connections.iter());
        let intersection: HashSet<_> = providers_to_remove
            .intersection(&providers_to_add)
            .map(|x| *x)
            .collect();

        providers_to_remove
            .difference(&intersection)
            .for_each(|x| _dom.remove_slot_from_provider(*x, &self.slot));
        providers_to_add
            .difference(&intersection)
            .for_each(|x| _dom.add_slot_to_provider(*x, &self.slot));

        // TODO: Use mem::swap here?
        slot.connections = self.connections.to_vec();

        // Generate render_graph mutation
        TransactionStepResult {
            changed_slots: vec![self.slot.clone()],
            graph_mutations: vec![],
        }
    }
}
