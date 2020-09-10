use std::sync::Arc;
use std::cell::{RefCell, Ref, RefMut};
use std::ops::{Deref, DerefMut};

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

// impl<T> Deref for Rf<T> {
//     type Target = Ref<'a, T>;
//
//     fn deref(&self) -> Ref<'_, T> {
//         *(self.reference).borrow()
//     }
// }
//
// impl<T> DerefMut for Rf<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut *(self.reference).borrow_mut()
//     }
// }
