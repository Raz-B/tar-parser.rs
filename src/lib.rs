#![warn(rust_2018_idioms)]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub use self::parser::*;

pub mod parser;
