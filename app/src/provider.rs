use crate::node::{ARef, Node};

enum ProviderType {
    Node(ARef<Node>),

}

struct Provider {
    ty: ProviderType,
}


