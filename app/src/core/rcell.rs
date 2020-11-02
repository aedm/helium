use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct RCell<T: ?Sized> {
    reference: Rc<RefCell<T>>,
}

// pub struct RWeak<T: ?Sized> {
//     reference: Weak<RefCell<T>>,
// }

impl<T> RCell<T> {
    pub fn new(t: T) -> RCell<T> {
        RCell {
            reference: Rc::new(RefCell::new(t)),
        }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        (*self.reference).borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        (*self.reference).borrow_mut()
    }

    // pub fn downgrade(&self) -> RWeak<T> {
    //     RWeak {
    //         reference: Rc::downgrade(&self.reference),
    //     }
    // }
}

impl<T: ?Sized> Debug for RCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Reference of {:?}", std::any::type_name::<T>())
    }
}

impl<T: ?Sized> Hash for RCell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reference.as_ptr().hash(state);
    }
}

impl<T: ?Sized> PartialEq for RCell<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.reference, &other.reference)
    }
}

impl<T: ?Sized> Clone for RCell<T> {
    fn clone(&self) -> Self {
        RCell {
            reference: self.reference.clone(),
        }
    }
}

impl<T: ?Sized> Eq for RCell<T> {}

// impl<T> RWeak<T> {
//     pub fn new() -> RWeak<T> {
//         RWeak {
//             reference: Weak::new(),
//         }
//     }
//
//     pub fn upgrade(&self) -> Option<RCell<T>> {
//         if let Some(rf) = self.reference.upgrade() {
//             Some(RCell { reference: rf })
//         } else {
//             None
//         }
//     }
// }
