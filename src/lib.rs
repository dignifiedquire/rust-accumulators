#![deny(clippy::all, clippy::perf, clippy::correctness)]
#![allow(clippy::unreadable_literal, clippy::many_single_char_names)]
#![warn(clippy::type_complexity, clippy::too_many_arguments)]


#[macro_use] extern crate serde_derive;
extern crate serde; 



pub mod hash;
pub mod math;
pub mod primes;
pub mod proofs;
pub mod rsa;
pub mod traits;
pub mod vc;
