use crate::node::{CoreProviderIndex, CoreSlotIndex};
use crate::node_ref::CoreNodeRef;
use crate::provider::CoreProvider;
use crate::rcell::RCell;
use crate::slot::CoreSlotDefault;
use std::mem;

pub struct CoreMutationSequence {
    pub steps: Vec<CoreMutation>,
}

pub enum CoreMutation {
    SetSlotConnections(SetSlotConnectionsParams),
    SetNodeDependencyList(SetNodeDependencyListParams),
    SetSlotDefaultValue(SetSlotDefaultValueParams),
}

pub struct SetSlotConnectionsParams {
    pub slot: CoreSlotIndex,
    pub connection: Vec<CoreProviderIndex>,
    pub swap_vector: Vec<RCell<CoreProvider>>,
}

pub struct SetNodeDependencyListParams {
    pub node: CoreNodeRef,
    pub dependency_list: Vec<CoreNodeRef>,
}

pub struct SetSlotDefaultValueParams {
    pub node: CoreNodeRef,
    pub slot_index: usize,
    pub value: CoreSlotDefault,
}

impl CoreMutation {
    fn run(&mut self) {
        match self {
            CoreMutation::SetSlotConnections(x) => x.run(),
            CoreMutation::SetNodeDependencyList(x) => x.run(),
            CoreMutation::SetSlotDefaultValue(x) => x.run(),
        }
    }
}

impl CoreMutationSequence {
    pub fn new(steps: Vec<CoreMutation>) -> CoreMutationSequence {
        CoreMutationSequence { steps }
    }

    pub fn run(&mut self) {
        for mutation in &mut self.steps {
            mutation.run();
        }
    }
}

impl SetSlotConnectionsParams {
    fn run(&mut self) {
        debug_assert_eq!(self.swap_vector.len(), 0);
        debug_assert_eq!(self.swap_vector.capacity(), self.connection.len());

        let node = self.slot.node.borrow_mut();
        let mut slot = node.descriptor().slots[self.slot.slot_index].borrow_mut();
        mem::swap(&mut slot.connection, &mut self.swap_vector);
        for connection in &self.connection {
            let provider_node = connection.node.borrow();
            let provider = provider_node.descriptor().providers[connection.provider_index].clone();
            if !slot.inner.can_connect(&provider.borrow()) {
                panic!(
                    "'{}:{}({:?})' slot can't connect to '{}:{}' provider.",
                    node.descriptor().name,
                    slot.name,
                    slot.inner.get_type(),
                    provider_node.descriptor().name,
                    provider.borrow().name
                );
            }
            slot.connection.push(provider);
        }

        debug_assert_eq!(slot.connection.len(), self.connection.len());
    }
}

impl SetNodeDependencyListParams {
    fn run(&mut self) {
        mem::swap(
            &mut self.node.borrow_mut().descriptor_mut().dependency_list,
            &mut self.dependency_list,
        );
    }
}

impl SetSlotDefaultValueParams {
    fn run(&mut self) {
        let node = self.node.borrow_mut();
        let mut slot = node.descriptor().slots[self.slot_index].borrow_mut();
        slot.set_default(&self.value);
    }
}
