#![warn(rust_2018_idioms)]
#![no_std]

extern crate alloc;

pub use self::parser::*;

pub mod parser;
