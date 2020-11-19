use crate::core::provider::CoreProvider;
use crate::core::rcell::RCell;

#[derive(Debug)]
pub enum SlotType {
    Float32,
    Node,
}

#[derive(Clone, Copy)]
pub enum CoreSlotDefault {
    None,
    Float32(f32),
}

pub trait CoreSlotInner {
    fn can_connect(self: &Self, provider: &CoreProvider) -> bool;
    fn get_type(self: &Self) -> SlotType;
}

pub struct CoreSlot {
    pub name: String,
    pub connection: Vec<RCell<CoreProvider>>,
    _allow_multiple: bool,
    pub inner: Box<dyn CoreSlotInner>,
    pub default: CoreSlotDefault,
}

impl CoreSlot {
    pub fn new(
        name: &str,
        allow_multiple: bool,
        inner: Box<dyn CoreSlotInner>,
        default: CoreSlotDefault,
    ) -> CoreSlot {
        CoreSlot {
            name: name.to_string(),
            connection: vec![],
            _allow_multiple: allow_multiple,
            inner,
            default,
        }
    }

    pub fn set_default(&mut self, default: &CoreSlotDefault) {
        self.default = *default;
    }

    pub fn get_single_provider(&self) -> Option<&RCell<CoreProvider>> {
        match self.connection.len() {
            0 => None,
            1 => Some(&self.connection[0]),
            _ => panic!("'get_single_provider' called, multiple providers connected."),
        }
    }
}
