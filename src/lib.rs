// Copyright 2018 Stichting Organism
//
// Copyright 2018 Friedel Ziegelmayer
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


#![deny(clippy::all, clippy::perf, clippy::correctness)]
#![allow(clippy::unreadable_literal, clippy::many_single_char_names)]
#![warn(clippy::type_complexity, clippy::too_many_arguments)]


#[macro_use] extern crate serde_derive;
extern crate serde; 
extern crate failure;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate lazy_static;
extern crate num_iter;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate rand;
extern crate byteorder;
extern crate blake2; 
extern crate classgroup;

pub mod hash;
pub mod math;
pub mod primes;
pub mod proofs;
pub mod rsa;
pub mod traits;
pub mod vc;
