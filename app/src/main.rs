use crate::core::core_mutation::{
    CoreMutationSequence, SetNodeDependencyListCoreMutation, SetSlotConnectionsCoreMutation,
};
use crate::core::node::{CoreNode, CoreProviderIndex, CoreSlotIndex};
use crate::core::slot::CoreSlotDefault;
use crate::flow::dom::Dom;
use crate::flow::flow_node::{FlowNode, FlowSlotIndex};
use crate::flow::mutation::{FlowMutation, FlowMutationStepResult};
use crate::flow::mutation_create_node::CreateNodeFlowMutation;
use crate::flow::mutation_set_connections::SetSlotConnectionsFlowMutation;
use crate::nodes::float_node::FloatNode;
use crate::nodes::sum_node::SumNode;
use crate::stillaxis::Stillaxis;
use flow::topological_order::TopologicalOrder;
use std::any::TypeId;
use std::borrow::Borrow;

mod core;
mod flow;
mod nodes;
mod stillaxis;

fn main() {

}
