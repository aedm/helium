use crate::core::node::NodeRef;

// Works on the engine graph
trait Mutation {
    fn run(&self);
}

struct MutationSequence {
    steps: Vec<Box<dyn Mutation>>,
}

struct CreateNodeMutation {
    new_node: NodeRef,
}

impl Mutation for CreateNodeMutation {
    fn run(&self) {
        unimplemented!()
    }
}

