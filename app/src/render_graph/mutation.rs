use crate::render_graph::node::{NodeProviderRef, NodeSlotRef};
use crate::render_graph::node_ref::NodeRef;
use crate::render_graph::provider::Provider;
use crate::render_graph::rcell::RCell;
use crate::render_graph::slot::SlotDefault;
use std::mem;

pub struct MutationSequence {
    pub steps: Vec<Mutation>,
}

pub enum Mutation {
    SetSlotConnections(SetSlotConnectionsParams),
    SetNodeDependencyList(SetNodeDependencyListParams),
    SetSlotDefaultValue(SetSlotDefaultValueParams),
}

pub struct SetSlotConnectionsParams {
    pub slot: NodeSlotRef,
    pub connection: Vec<NodeProviderRef>,
    pub swap_vector: Vec<RCell<Provider>>,
}

pub struct SetNodeDependencyListParams {
    pub node: NodeRef,
    pub dependency_list: Vec<NodeRef>,
}

pub struct SetSlotDefaultValueParams {
    pub node: NodeRef,
    pub slot_index: usize,
    pub value: SlotDefault,
}

impl Mutation {
    fn run(&mut self) {
        match self {
            Mutation::SetSlotConnections(x) => x.run(),
            Mutation::SetNodeDependencyList(x) => x.run(),
            Mutation::SetSlotDefaultValue(x) => x.run(),
        }
    }
}

impl MutationSequence {
    pub fn new(steps: Vec<Mutation>) -> MutationSequence {
        MutationSequence { steps }
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
