use crate::core::node::NodeId;
// use crate::nodes::float_node::FloatNode;
//
// enum NodeType {
//     Float,
//     Sum,
// }
//
// enum ProviderType {
//     _None,
//     Float,
// }
//
// struct DomSlot {
//
// }
//
// pub struct DomNode {
//     id: NodeId,
//
// }

// pub struct Mutation {
//
// }


struct ConnectSingleNodeCommand {
    source_node_id: NodeId,
    source_provider_index: usize,
    target_node_id: NodeId,
    target_slot_index: usize,
}
