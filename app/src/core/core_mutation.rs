use crate::core::node::{CoreNodeRef, CoreProviderIndex, CoreSlotIndex};
use crate::core::acell::ACell;
use crate::core::slot::CoreSlot;
use std::mem;
use crate::core::provider::CoreProvider;

pub trait CoreMutation {
    fn run(&mut self);
}

pub struct CoreMutationSequence {
    pub steps: Vec<Box<dyn CoreMutation>>,
}

impl CoreMutationSequence {
    pub fn new(steps: Vec<Box<dyn CoreMutation>>) -> CoreMutationSequence {
        CoreMutationSequence { steps }
    }

    pub fn run(&mut self) {
        for mutation in &mut self.steps {
            mutation.run();
        }
    }
}

pub struct SetSlotConnectionsCoreMutation {
    pub slot: CoreSlotIndex,
    pub connection: Vec<CoreProviderIndex>,
    pub swap_vector: Vec<ACell<CoreProvider>>,
}

impl CoreMutation for SetSlotConnectionsCoreMutation {
    fn run(&mut self) {
        debug_assert_eq!(self.swap_vector.len(), 0);
        debug_assert_eq!(self.swap_vector.capacity(), self.connection.len());

        let mut node = self.slot.node.borrow_mut();
        let mut slot = node.slots[self.slot.slot_index].borrow_mut();
        mem::swap(&mut slot.connection, &mut self.swap_vector);
        for connection in &self.connection {
            let provider = connection.node.borrow().providers[connection.provider_index].clone();
            slot.connection.push(provider);
        }

        debug_assert_eq!(slot.connection.len(), self.connection.len());
    }
}

pub struct SetNodeDependencyListCoreMutation {
    pub node: CoreNodeRef,
    pub dependency_list: Vec<CoreNodeRef>,
}

impl CoreMutation for SetNodeDependencyListCoreMutation {
    fn run(&mut self) {
        mem::swap(
            &mut self.node.borrow_mut().dependency_list,
            &mut self.dependency_list,
        );
    }
}
