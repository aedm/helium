use super::dom::FlowDom;
use super::flow_node::FlowSlotIndex;
use super::mutation::{FlowMutationStep, FlowMutationStepResult};
use crate::flow::flow_node::FlowProviderIndex;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct SetSlotConnectionsFlowMutation {
    pub node_slot: FlowSlotIndex,
    pub connections: Vec<FlowProviderIndex>,
}

impl SetSlotConnectionsFlowMutation {
    pub fn new(
        node_slot: FlowSlotIndex,
        connections: Vec<FlowProviderIndex>,
    ) -> Box<SetSlotConnectionsFlowMutation> {
        Box::new(SetSlotConnectionsFlowMutation {
            node_slot,
            connections,
        })
    }
}

impl FlowMutationStep for SetSlotConnectionsFlowMutation {
    fn run(&self, _dom: &mut FlowDom) -> FlowMutationStepResult {
        // Change flow DOM
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

        // Generate core mutation
        FlowMutationStepResult {
            changed_slots: vec![self.node_slot.clone()],
            core_mutations: vec![],
        }
    }
}
