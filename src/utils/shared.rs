use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// A convenience wrapper for values that exhibit
/// shared (reference-counted) ownership and interior
/// mutability.
/// 
/// It is not thread-safe.
pub type Shared<T> = Rc<RefCell<T>>;

pub type WeakShared<T> = Weak<RefCell<T>>;

pub fn share<T>(value: T) -> Shared<T> {
	Rc::new(RefCell::new(value))
}
