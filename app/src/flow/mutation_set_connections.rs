use super::dom::FlowDom;
use super::flow_node::FlowSlotIndex;
use super::mutation::{FlowMutationStep, FlowMutationStepResult};
use crate::flow::flow_node::{FlowNodeRef, FlowSlot, FlowProviderIndex};

pub struct SetSlotConnectionsFlowMutation {
    pub node_slot: FlowSlotIndex,
    pub connections: Vec<FlowProviderIndex>,
}

impl SetSlotConnectionsFlowMutation {
    pub fn new_single(
        slot: FlowSlotIndex,
        provider_node: &FlowNodeRef,
        provider_index: usize,
    ) -> Box<SetSlotConnectionsFlowMutation> {
        Box::new(SetSlotConnectionsFlowMutation {
            node_slot: slot,
            connections: vec![FlowProviderIndex {
                node: provider_node.clone(),
                provider_index
            }],
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
