use crate::core::node::CoreNodeRef;
use crate::core::rf::ACell;
use crate::core::slot::{CoreSlot, CoreSlotConnection};
use std::mem;

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
    pub slot: ACell<CoreSlot>,
    pub connection: CoreSlotConnection,
}

impl CoreMutation for SetSlotConnectionsCoreMutation {
    fn run(&mut self) {
        mem::swap(&mut self.slot.borrow_mut().connection, &mut self.connection);
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
