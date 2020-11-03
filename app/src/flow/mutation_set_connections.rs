use super::dom::FlowDom;
use super::flow_node::FlowSlotIndex;
use super::mutation::{FlowMutationStep, FlowMutationStepResult};
use crate::flow::flow_node::FlowNodeRef;

pub struct SetSlotConnectionsFlowMutation {
    pub node_slot: FlowSlotIndex,
    pub connections: Vec<FlowSlotIndex>,
}

impl SetSlotConnectionsFlowMutation {
    pub fn new_single(
        slot_node: &FlowNodeRef,
        slot_index: usize,
        provider_node: &FlowNodeRef,
        provider_index: usize,
    ) -> Box<SetSlotConnectionsFlowMutation> {
        Box::new(SetSlotConnectionsFlowMutation {
            node_slot: FlowSlotIndex {
                node: slot_node.clone(),
                slot_index,
            },
            connections: vec![FlowSlotIndex {
                node: provider_node.clone(),
                slot_index: provider_index,
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
        }
    }
}
