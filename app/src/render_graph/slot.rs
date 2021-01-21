use crate::render_graph::provider::Provider;
use crate::render_graph::rcell::RCell;

#[derive(Debug)]
pub enum SlotType {
    Float32,
    Node,
}

#[derive(Clone, Copy)]
pub enum SlotDefault {
    None,
    Float32(f32),
}

pub trait SlotInner {
    fn can_connect(self: &Self, provider: &Provider) -> bool;
    fn get_type(self: &Self) -> SlotType;
}

pub struct Slot {
    pub name: String,
    pub connection: Vec<RCell<Provider>>,
    _allow_multiple: bool,
    pub inner: Box<dyn SlotInner>,
    pub default: SlotDefault,
}

impl Slot {
    pub fn new(
        name: &str,
        allow_multiple: bool,
        inner: Box<dyn SlotInner>,
        default: SlotDefault,
    ) -> Slot {
        Slot {
            name: name.to_string(),
            connection: vec![],
            _allow_multiple: allow_multiple,
            inner,
            default,
        }
    }

    pub fn set_default(&mut self, default: &SlotDefault) {
        self.default = *default;
    }

    pub fn get_single_provider(&self) -> Option<&RCell<Provider>> {
        match self.connection.len() {
            0 => None,
            1 => Some(&self.connection[0]),
            _ => panic!("'get_single_provider' called, multiple providers connected."),
        }
    }
}
