// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*! Task-local garbage-collected boxes

The `Gc` type provides shared ownership of an immutable value. Destruction is not deterministic, and
will occur some time between every `Gc` handle being gone and the end of the task. The garbage
collector is task-local so `Gc<T>` is not sendable.

*/

#![allow(experimental)]

use kinds::marker;
use clone::Clone;
use managed;

/// Immutable garbage-collected pointer type
#[lang="gc"]
#[cfg(not(test))]
#[experimental = "Gc is currently based on reference-counting and will not collect cycles until \
                  task annihilation. For now, cycles need to be broken manually by using `Rc<T>` \
                  with a non-owning `Weak<T>` pointer. A tracing garbage collector is planned."]
pub struct Gc<T> {
    ptr: @T,
    marker: marker::NoSend,
}

#[cfg(test)]
pub struct Gc<T> {
    ptr: @T,
    marker: marker::NoSend,
}

impl<T: 'static> Gc<T> {
    /// Construct a new garbage-collected box
    #[inline]
    pub fn new(value: T) -> Gc<T> {
        Gc { ptr: @value, marker: marker::NoSend }
    }

    /// Borrow the value contained in the garbage-collected box
    #[inline]
    pub fn borrow<'r>(&'r self) -> &'r T {
        &*self.ptr
    }

    /// Determine if two garbage-collected boxes point to the same object
    #[inline]
    pub fn ptr_eq(&self, other: &Gc<T>) -> bool {
        managed::ptr_eq(self.ptr, other.ptr)
    }
}

impl<T> Clone for Gc<T> {
    /// Clone the pointer only
    #[inline]
    fn clone(&self) -> Gc<T> {
        Gc{ ptr: self.ptr, marker: marker::NoSend }
    }
}

/// An value that represents the task-local managed heap.
///
/// Use this like `let foo = box(GC) Bar::new(...);`
#[lang="managed_heap"]
#[cfg(not(test))]
pub static GC: () = ();

#[cfg(test)]
pub static GC: () = ();

#[cfg(test)]
mod tests {
    use prelude::*;
    use super::*;
    use cell::RefCell;

    #[test]
    fn test_clone() {
        let x = Gc::new(RefCell::new(5));
        let y = x.clone();
        *x.borrow().borrow_mut() = 20;
        assert_eq!(*y.borrow().borrow(), 20);
    }

    #[test]
    fn test_simple() {
        let x = Gc::new(5);
        assert_eq!(*x.borrow(), 5);
    }

    #[test]
    fn test_simple_clone() {
        let x = Gc::new(5);
        let y = x.clone();
        assert_eq!(*x.borrow(), 5);
        assert_eq!(*y.borrow(), 5);
    }

    #[test]
    fn test_ptr_eq() {
        let x = Gc::new(5);
        let y = x.clone();
        let z = Gc::new(7);
        assert!(x.ptr_eq(&x));
        assert!(x.ptr_eq(&y));
        assert!(!x.ptr_eq(&z));
    }

    #[test]
    fn test_destructor() {
        let x = Gc::new(box 5);
        assert_eq!(**x.borrow(), 5);
    }
}
