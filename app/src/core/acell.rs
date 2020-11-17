use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub struct ACell<T: ?Sized> {
    reference: Arc<RefCell<T>>,
}

// pub struct AWeak<T: ?Sized> {
//     _reference: sync::Weak<RefCell<T>>,
// }

impl<T: ?Sized> ACell<dyn T> {
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

    pub fn refc(&self) -> usize {
        Arc::strong_count(&self.reference)
    }

    // pub fn downgrade(&self) -> AWeak<T> {
    //     AWeak {
    //         _reference: Arc::downgrade(&self.reference),
    //     }
    // }
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

unsafe impl<T: ?Sized> Send for ACell<T> {}

// impl<T> AWeak<T> {
//     pub fn new() -> AWeak<T> {
//         AWeak {
//             _reference: sync::Weak::new(),
//         }
//     }
//
//     pub fn upgrade(&self) -> Option<ACell<T>> {
//         if let Some(rf) = self._reference.upgrade() {
//             Some(ACell { reference: rf })
//         } else {
//             None
//         }
//     }
// }
