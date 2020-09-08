use crate::node::ARef;
use std::pin::Pin;

pub struct Texture {
}

pub struct Mesh {
}

pub enum Value {
    Float32(f32),
    Int64(i64),
    Bool(bool),
    Texture(ARef<Texture>),
    Mesh(ARef<Texture>),
}
