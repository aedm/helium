use std::sync::Arc;
use std::cell::{RefCell, Ref, RefMut};
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Formatter};
use std::{fmt, sync};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

pub struct Rf<T: ?Sized> {
    reference: Arc<RefCell<T>>,
}

pub struct Weak<T: ?Sized> {
    reference: sync::Weak<RefCell<T>>,
}

impl<T> Rf<T> {
    pub fn new(t: T) -> Rf<T> {
        Rf {
            reference: Arc::new(RefCell::new(t)),
        }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        (*self.reference).borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        (*self.reference).borrow_mut()
    }

    pub fn clone(&self) -> Rf<T> {
        Rf {
            reference: self.reference.clone()
        }
    }

    pub fn downgrade(&self) -> Weak<T> {
        Weak {
            reference: Arc::downgrade(&self.reference),
        }
    }
}

impl<T: ?Sized> PartialEq for Rf<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.reference, &other.reference)
    }
}

impl<T: ?Sized> Debug for Rf<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Reference of {:?}", std::any::type_name::<T>())
    }
}

// impl<T: ?Sized> Ord for Rf<T> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         unimplemented!()
//     }
// }

impl<T: ?Sized> Hash for Rf<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.hash(state);
    }
}

impl<T: ?Sized> Eq for Rf<T> {}

impl<T> Weak<T> {
    pub fn new() -> Weak<T> {
        Weak {
            reference: sync::Weak::new(),
        }
    }

    fn upgrade(&self) -> Option<Rf<T>> {
        if let Some(rf) = self.reference.upgrade() {
            Some(Rf { reference: rf })
        } else { None }
    }
}