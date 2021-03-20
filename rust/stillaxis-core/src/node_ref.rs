use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Weak};
use crate::node::CoreNode;

pub struct CoreNodeRef {
    reference: Arc<RefCell<Box<dyn CoreNode>>>,
}

pub struct CoreNodeWeak {
    reference: Weak<RefCell<Box<dyn CoreNode>>>,
}

impl CoreNodeRef {
    pub fn new(node: Box<dyn CoreNode>) -> CoreNodeRef {
        CoreNodeRef {
            reference: Arc::new(RefCell::new(node)),
        }
    }

    pub fn borrow(&self) -> Ref<'_, Box<dyn CoreNode>> {
        (*self.reference).borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, Box<dyn CoreNode>> {
        (*self.reference).borrow_mut()
    }

    pub fn clone(&self) -> CoreNodeRef {
        CoreNodeRef {
            reference: self.reference.clone(),
        }
    }

    pub fn refc(&self) -> usize {
        Arc::strong_count(&self.reference)
    }

    pub fn downgrade(&self) -> CoreNodeWeak {
        CoreNodeWeak {
            reference: Arc::downgrade(&self.reference),
        }
    }
}

impl CoreNodeWeak {
    pub fn new() -> CoreNodeWeak {
        CoreNodeWeak {
            reference: Weak::new(),
        }
    }

    pub fn upgrade(&self) -> Option<CoreNodeRef> {
        if let Some(rf) = self.reference.upgrade() {
            Some(CoreNodeRef { reference: rf })
        } else {
            None
        }
    }
}

impl Debug for CoreNodeRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let x = self.borrow();
        let y = x.descriptor().fmt(f);
        write!(f, "ref->{:?}", y)
    }
}

impl Hash for CoreNodeRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.as_ptr().hash(state);
    }
}

impl PartialEq for CoreNodeRef {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.reference, &other.reference)
    }
}

impl Eq for CoreNodeRef {}

unsafe impl Send for CoreNodeRef {}

impl Debug for CoreNodeWeak {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(node) = self.upgrade() {
            let _ = write!(f, "weak->");
            node.borrow().descriptor().fmt(f)
        } else {
            write!(f, "weak->nothing")
        }
    }
}

impl Clone for CoreNodeWeak {
    fn clone(&self) -> Self {
        CoreNodeWeak {
            reference: self.reference.clone(),
        }
    }
}

impl PartialEq for CoreNodeWeak {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.reference, &other.reference)
    }
}

impl Eq for CoreNodeWeak {}
