use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Address(pub usize);

/// # Address to Generic Types
///   The type of a new `Address` is holding must always
/// implement the `Copy` trait.
///   When making an `Address` for a struct (call it `Foo`),
/// please always make sure you're giving it a reference (`&Foo`),
/// since references implement the `Copy` trait.
#[allow(clippy::inline_always)]
impl Address {
    #[inline(always)]
    pub fn new<T : Copy>(structure : T) -> Self {
        Self(Box::into_raw(Box::new(structure)) as usize)
    }

    /// # Safety
    /// This function dereferences a raw pointer.
    #[inline(always)]
    #[must_use]
    pub unsafe fn deref<T : Copy>(self) -> T {
        *(self.0 as *mut T)
    }

    // For temporary use.
    #[inline(always)]
    #[must_use]
    pub fn null() -> Self { Address(0) }
}

impl fmt::Debug for Address {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "void *0x{:016X}", self.0)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Address(0x{:016X})", self.0)
    }
}


// Perhaps figure out this overload in the future.
/*
use std::ops::Deref;

impl<T> Deref for Address {
    type Target = T;

    fn deref(&self) -> Self::Target {
        *(self.0 as *const T)
    }
}
*/