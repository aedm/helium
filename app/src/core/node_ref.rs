use crate::core::node::CoreNode;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct CoreNodeRef {
    reference: Arc<RefCell<Box<dyn CoreNode>>>,
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
}

impl Debug for CoreNodeRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let x = self.borrow();
        let y = x.get_inner().fmt(f);
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
