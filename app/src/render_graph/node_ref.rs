use crate::render_graph::node::Node;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Weak};

pub struct NodeRef {
    reference: Arc<RefCell<Box<dyn Node>>>,
}

pub struct NodeWeak {
    reference: Weak<RefCell<Box<dyn Node>>>,
}

impl NodeRef {
    pub fn new(node: Box<dyn Node>) -> NodeRef {
        NodeRef {
            reference: Arc::new(RefCell::new(node)),
        }
    }

    pub fn borrow(&self) -> Ref<'_, Box<dyn Node>> {
        (*self.reference).borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Box<dyn Node>> {
        (*self.reference).borrow_mut()
    }

    pub fn clone(&self) -> NodeRef {
        NodeRef {
            reference: self.reference.clone(),
        }
    }

    pub fn refc(&self) -> usize {
        Arc::strong_count(&self.reference)
    }

    pub fn downgrade(&self) -> NodeWeak {
        NodeWeak {
            reference: Arc::downgrade(&self.reference),
        }
    }
}

impl NodeWeak {
    pub fn new() -> NodeWeak {
        NodeWeak {
            reference: Weak::new(),
        }
    }

    pub fn upgrade(&self) -> Option<NodeRef> {
        if let Some(rf) = self.reference.upgrade() {
            Some(NodeRef { reference: rf })
        } else {
            None
        }
    }
}

impl Debug for NodeRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let x = self.borrow();
        let y = x.descriptor().fmt(f);
        write!(f, "ref->{:?}", y)
    }
}

impl Hash for NodeRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.as_ptr().hash(state);
    }
}

impl PartialEq for NodeRef {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.reference, &other.reference)
    }
}

impl Eq for NodeRef {}

unsafe impl Send for NodeRef {}

impl Debug for NodeWeak {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(node) = self.upgrade() {
            let _ = write!(f, "weak->");
            node.borrow().descriptor().fmt(f)
        } else {
            write!(f, "weak->nothing")
        }
    }
}

impl Clone for NodeWeak {
    fn clone(&self) -> Self {
        NodeWeak {
            reference: self.reference.clone(),
        }
    }
}

impl PartialEq for NodeWeak {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.reference, &other.reference)
    }
}

impl Eq for NodeWeak {}
