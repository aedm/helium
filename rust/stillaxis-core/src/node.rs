use crate::node_descriptor::{NodeDescriptor, NodeId};
use crate::node_ref::NodeRef;
use std::fmt::Debug;

pub struct ProviderRef {
    pub node: NodeRef,
    pub provider_index: usize,
}

pub struct SlotRef {
    pub node: NodeRef,
    pub slot_index: usize,
}

pub trait Node: Debug {
    fn new(id: NodeId) -> Self
    where
        Self: std::marker::Sized;

    fn descriptor(&self) -> &NodeDescriptor;
    fn descriptor_mut(&mut self) -> &mut NodeDescriptor;

    fn run(&mut self);
}
