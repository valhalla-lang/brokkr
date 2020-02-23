use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Address(pub usize);

/// # Address to any type
/// Pass only objects with size known at compile time to `new`.
#[allow(clippy::inline_always)]
impl Address {
    #[inline(always)]
    pub fn new<T>(structure : T) -> Self {
        Self(Box::into_raw(Box::new(structure)) as usize)
    }

    /// # Safety
    /// This function dereferences a raw pointer.
    #[inline(always)]
    #[must_use]
    pub unsafe fn deref<T : Copy>(self) -> T {
        *(self.0 as *mut T)
    }

    /// # Safety
    /// This function dereferences a raw pointer.
    #[inline(always)]
    #[must_use]
    pub unsafe fn reference<T>(self) -> &'static T {
        &*(self.0 as *mut T)
    }

    /// Get value of self.0, if it does not represent a pointer,
    /// and instead is a value such as f64, stored as a usize.
    /// # Safety
    /// This function transmutes types.
    #[inline(always)]
    #[must_use]
    pub unsafe fn value<T>(self) -> T {
        std::mem::transmute_copy(&self.0)
    }

    /// # Safety
    /// This function transmutes types.
    #[inline(always)]
    #[must_use]
    pub unsafe fn from_value<T>(value : &T) -> Self {
        std::mem::transmute_copy(value)
    }

    /// Null-pointer, for temporary use.
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
