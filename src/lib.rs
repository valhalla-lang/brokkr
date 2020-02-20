#![allow(incomplete_features)]
#![feature(const_generics)]
#![warn(
    clippy::all,
    clippy::pedantic,
)]
#![allow(clippy::needless_return)]

pub mod assembler;
pub mod vm;

pub fn main() { std::process::exit(1); }
