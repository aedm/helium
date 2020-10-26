use super::flow_node::FlowSlotIndex;
use super::mutation::{FlowMutationStep, FlowMutationStepResult};
use super::dom::Dom;

pub struct SetSlotConnectionsFlowMutation {
    pub node_slot: FlowSlotIndex,
    pub connections: Vec<FlowSlotIndex>,
}

impl FlowMutationStep for SetSlotConnectionsFlowMutation {
    fn run(&self, dom: &mut Dom) -> FlowMutationStepResult {
        // Change shadow DOM
        let mut node = self.node_slot.node.borrow_mut();
        node.slots[self.node_slot.slot_index].connections = self.connections.to_vec();

        // Generate core mutation
        FlowMutationStepResult {
            changed_slots: vec![self.node_slot.clone()],
        }
    }
}
