use std::sync::Arc;
use std::cell::{RefCell, Ref, RefMut};
use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Formatter};
use std::fmt;

pub struct Rf<T: ?Sized> {
    reference: Arc<RefCell<T>>,
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
}

impl<T: ?Sized> Debug for Rf<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Reference of {:?}", std::any::type_name::<T>())
    }
}

