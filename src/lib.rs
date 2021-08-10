#![warn(rust_2018_idioms)]
#![cfg_attr(all(not(feature = "std"), feature = "alloc"), feature(alloc))]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

extern crate alloc;

pub use self::parser::*;

pub mod parser;
