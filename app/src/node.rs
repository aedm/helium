use std::sync::Arc;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::ops::Deref;

pub type ARef<T> = Arc<RefCell<T>>;

fn new_aref<T>(t: T) -> ARef<T> {
    Arc::new(RefCell::new(t))
}

enum ResourceType {
    Float32,
}

enum InputConnection {
    None,
    Single(ARef<OutputSlot>),
    Multi(Vec<ARef<OutputSlot>>),
}

struct InputSlot {
    name: String,
    resource_type: ResourceType,
    allow_multiple: bool,
    connection: InputConnection,
}

impl InputSlot {
    fn new(name: &str, resource_type: ResourceType, allow_multiple: bool) -> InputSlot {
        InputSlot {
            name: name.to_string(),
            resource_type,
            allow_multiple,
            connection: InputConnection::None,
        }
    }

    fn get_single(self: &Self) -> Option<ARef<OutputSlot>> {
        if let InputConnection::Single(x) = &self.connection {
            return Some(x.clone())
        }
        None
    }
}

enum OutputSlotConnection {
    None,
    Single(ARef<InputSlot>),
    Multi(Vec<ARef<InputSlot>>),
}

struct OutputSlot {
    name: String,
    resource_type: ResourceType,
    connection: OutputSlotConnection,
}

impl OutputSlot {
    fn new(name: &str, resource_type: ResourceType) -> OutputSlot {
        OutputSlot {
            name: name.to_string(),
            resource_type,
            connection: OutputSlotConnection::None,
        }
    }
}

trait NodeInner {
    fn init(self: &mut Self, node: &mut Node);
    fn run(self: &mut Self);
}

pub struct Node {
    input_slots: Vec<ARef<InputSlot>>,
    output_slots: Vec<ARef<OutputSlot>>,
    inner: Box<dyn NodeInner>,
}

impl Node {
    fn new(inner: Box<dyn NodeInner>) -> Node {
        Node {
            input_slots: vec![],
            output_slots: vec![],
            inner,
        }
    }
}

struct Adder {
    a: ARef<InputSlot>,
    b: ARef<InputSlot>,
    res: ARef<OutputSlot>,
}

impl Adder {
    fn new() -> Adder {
        Adder {
            a: new_aref(InputSlot::new("A", ResourceType::Float32, false)),
            b: new_aref(InputSlot::new("B", ResourceType::Float32, false)),
            res: new_aref(OutputSlot::new("Sum", ResourceType::Float32)),
        }
    }
}

impl NodeInner for Adder {
    fn init(self: &mut Self, node: &mut Node) {
        unimplemented!()
    }

    fn run(self: &mut Self) {
        if let Some(x) = (self.a.deref().borrow()).get_single() {

        }
        // if let InputConnection::Single(a_ref) = self.a.borrow().connection {
        //     if let InputConnection::Single(b_ref) = &self.b.borrow().connection {
        //         a_ref.borrow().name;
        //     }
        // }
    }
}