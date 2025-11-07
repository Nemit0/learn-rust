//! Section: Smart Pointers (Box, Rc)
//!
//! Implement a tiny cons list using `Box` and practice `Rc` sharing.

#[derive(Debug, PartialEq, Eq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    /// Create a list by prepending a value to an existing list.
    pub fn cons(_head: i32, _tail: List) -> List {
        // TODO: Return List::Cons(head, Box::new(tail))
        unimplemented!("construct a cons cell with Box")
    }

    /// Return the number of elements in the list.
    pub fn len(&self) -> usize {
        // TODO: recursively count nodes
        unimplemented!("return 0 for Nil; 1 + len(tail) for Cons")
    }

    /// Convert the list to a Vec in head-to-tail order.
    pub fn to_vec(&self) -> Vec<i32> {
        // TODO: collect values into a Vec
        unimplemented!("iterate recursively and collect values")
    }
}

use std::rc::Rc;

/// Return a new reference-counted pointer to the given value.
pub fn make_rc(_value: i32) -> Rc<i32> {
    // TODO: Rc::new(value)
    unimplemented!("wrap the value in Rc::new")
}

/// Clone the Rc (incrementing the strong count) and return the clone.
pub fn clone_rc(_rc: &Rc<i32>) -> Rc<i32> {
    // TODO: Rc::clone(rc)
    unimplemented!("clone the Rc to increase strong count")
}

/// Get the current strong count of an Rc.
pub fn strong_count(_rc: &Rc<i32>) -> usize {
    // TODO: Rc::strong_count(rc)
    unimplemented!("return Rc::strong_count(rc)")
}

