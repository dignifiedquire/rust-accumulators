#![deny(clippy::all, clippy::perf, clippy::correctness)]
#![allow(clippy::unreadable_literal, clippy::many_single_char_names)]
#![warn(clippy::type_complexity, clippy::too_many_arguments)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

// #[macro_use] extern crate failure_derive;

#[cfg(feature = "class_group")]
extern crate classygroup;

pub mod accumulator;
pub mod group;
pub mod hash;
pub mod math;
pub mod proofs;
pub mod traits;
pub mod vc;

pub use self::accumulator::*;
pub use self::traits::*;
pub use self::vc::*;
