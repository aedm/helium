use super::dom::FlowDom;
use super::flow_node::FlowSlotIndex;
use super::mutation::{FlowMutationStep, FlowMutationStepResult};
use crate::flow::flow_node::FlowProviderIndex;

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
        // Change shadow DOM
        let mut node = self.node_slot.node.borrow_mut();
        node.slots[self.node_slot.slot_index].connections = self.connections.to_vec();

        // Generate core mutation
        FlowMutationStepResult {
            changed_slots: vec![self.node_slot.clone()],
            core_mutations: vec![],
        }
    }
}
