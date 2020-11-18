use crate::core::node_ref::CoreNodeRef;

use std::fmt::Debug;

use crate::core::core_node_descriptor::{CoreNodeDescriptor, NodeId};

pub struct CoreProviderIndex {
    pub node: CoreNodeRef,
    pub provider_index: usize,
}

pub struct CoreSlotIndex {
    pub node: CoreNodeRef,
    pub slot_index: usize,
}

pub trait CoreNode: Debug {
    fn new(id: NodeId) -> Self
    where
        Self: std::marker::Sized;

    fn descriptor(&self) -> &CoreNodeDescriptor;
    fn descriptor_mut(&mut self) -> &mut CoreNodeDescriptor;

    fn run(&mut self);
}
