#![deny(clippy::all, clippy::perf, clippy::correctness)]
#![allow(clippy::unreadable_literal, clippy::many_single_char_names)]
#![warn(clippy::type_complexity, clippy::too_many_arguments)]

#[macro_use]
extern crate serde_derive;
// #[macro_use] extern crate failure_derive;

#[cfg(feature = "class_group")]
extern crate classgroup;

pub mod accumulator;
pub mod group;
pub mod hash;
pub mod math;
pub mod proofs;
pub mod traits;
pub mod vc;
