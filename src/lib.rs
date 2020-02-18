#![allow(incomplete_features)]
#![feature(const_generics)]
 #![warn(
     clippy::all,
     clippy::pedantic,
 )]
#![allow(clippy::needless_return)]

use std::ffi::c_void;
type VoidPtr = *const c_void;

pub mod assembler;
pub mod vm;

pub fn main() { std::process::exit(0); }
