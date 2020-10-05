use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::{fmt, sync};

pub struct ACell<T: ?Sized> {
    reference: Arc<RefCell<T>>,
}

pub struct Weak<T: ?Sized> {
    reference: sync::Weak<RefCell<T>>,
}

impl<T> ACell<T> {
    pub fn new(t: T) -> ACell<T> {
        ACell {
            reference: Arc::new(RefCell::new(t)),
        }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        (*self.reference).borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        (*self.reference).borrow_mut()
    }

    pub fn clone(&self) -> ACell<T> {
        ACell {
            reference: self.reference.clone(),
        }
    }

    pub fn downgrade(&self) -> Weak<T> {
        Weak {
            reference: Arc::downgrade(&self.reference),
        }
    }
}

impl<T: ?Sized> Debug for ACell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Reference of {:?}", std::any::type_name::<T>())
    }
}

impl<T: ?Sized> Hash for ACell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.as_ptr().hash(state);
    }
}

impl<T: ?Sized> PartialEq for ACell<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.reference, &other.reference)
    }
}

impl<T: ?Sized> Eq for ACell<T> {}

impl<T> Weak<T> {
    pub fn new() -> Weak<T> {
        Weak {
            reference: sync::Weak::new(),
        }
    }

    pub fn upgrade(&self) -> Option<ACell<T>> {
        if let Some(rf) = self.reference.upgrade() {
            Some(ACell { reference: rf })
        } else {
            None
        }
    }
}