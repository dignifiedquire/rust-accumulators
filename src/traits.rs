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

use num_bigint::{BigInt, BigUint};
use rand::Rng;

pub trait StaticAccumulator {
    /// Setup generates a group of unknown order and initializes the group
    /// with a generator of that group.
    fn setup(rng: &mut impl Rng, lambda: usize) -> Self;

    /// Update the accumulator.
    fn add(&mut self, x: &BigUint);

    /// Create a membership proof.
    /// Returns `None`, iff `x` is not a member.
    fn mem_wit_create(&self, x: &BigUint) -> BigUint;

    /// Verify a membership proof.
    fn ver_mem(&self, w: &BigUint, x: &BigUint) -> bool;
}

pub trait DynamicAccumulator: StaticAccumulator {
    /// Delete a value from the accumulator.
    fn del(&mut self, x: &BigUint) -> Option<()>;
}

pub trait UniversalAccumulator: DynamicAccumulator {
    /// Create a non-membership proof.
    /// Returns `None`, iff `x` is a member.
    fn non_mem_wit_create(&self, x: &BigUint) -> (BigUint, BigInt);

    /// Verify a non-membership proof.
    fn ver_non_mem(&self, w: &(BigUint, BigInt), x: &BigUint) -> bool;
}

pub trait BatchedAccumulator: StaticAccumulator {
    /// Batch add.
    /// Given a list of new elements, adds them.
    fn batch_add(&mut self, xs: &[BigUint]) -> BigUint;

    /// Batch delete.
    /// Given a list of witnesses and members, deletes all of them.
    fn batch_del(&mut self, pairs: &[(BigUint, BigUint)]) -> Option<BigUint>;

    /// Delete with member witness.
    /// Deletes a single element, given the element and a wittness for it.
    /// Returns `None` if the element was not actual a member.
    fn del_w_mem(&mut self, w: &BigUint, x: &BigUint) -> Option<()>;

    /// Create membership witnesses for all elements in `s`.
    /// Needs to be passed in, as we don't hold onto the whole set in the accumulator currently.
    fn create_all_mem_wit(&self, s: &[BigUint]) -> Vec<BigUint>;

    /// Verify Batch Add.
    /// Given the proof `w` from [batch_add] and the list of members `xs`,
    /// and the previous state of the accumulator `a_t` this verifies if the `add` was done correctly.
    ///
    /// Note: This is not explicitly defined in the paper, but here for convenience.
    fn ver_batch_add(&self, w: &BigUint, a_t: &BigUint, xs: &[BigUint]) -> bool;

    /// Verify Batch Del
    /// Given the proof `w` from [batch_del] and the list of members `xs`,
    /// and the previous state of the accumulator `a_t` this verifies if the `del` was done correctly.
    ///
    /// Note: This is not explicitly defined in the paper, but here for convenience.
    fn ver_batch_del(&self, w: &BigUint, a_t: &BigUint, xs: &[BigUint]) -> bool;

    /// Aggregate two membership wittnesses, from the same accumulator.
    fn agg_mem_wit(
        &self,
        w_x: &BigUint,
        w_y: &BigUint,
        x: &BigUint,
        y: &BigUint,
    ) -> (BigUint, BigUint);

    /// Verify an aggregated membership wittness.
    fn ver_agg_mem_wit(&self, w_xy: &BigUint, pi: &BigUint, x: &BigUint, y: &BigUint) -> bool;

    /// Create a membership wittness for `x` and a NI-PoE for it.
    fn mem_wit_create_star(&self, x: &BigUint) -> (BigUint, BigUint);

    /// Verify a membership wittness with a NI-PoE.
    fn ver_mem_star(&self, x: &BigUint, pi: &(BigUint, BigUint)) -> bool;

    /// Aggregate two membership witness, from different accumulators.
    fn mem_wit_x(
        &self,
        other: &BigUint,
        w_x: &BigUint,
        w_y: &BigUint,
        x: &BigUint,
        y: &BigUint,
    ) -> BigUint;

    /// Verify aggregated membership witness.
    fn ver_mem_x(&self, other: &BigUint, pi: &BigUint, x: &BigUint, y: &BigUint) -> bool;

    /// Efficient non membership proof.
    fn non_mem_wit_create_star(
        &self,
        x: &BigUint,
    ) -> (BigUint, BigUint, (BigUint, BigUint, BigInt), BigUint);

    /// Verify non membership proof.
    fn ver_non_mem_star(
        &self,
        x: &BigUint,
        pi: &(BigUint, BigUint, (BigUint, BigUint, BigInt), BigUint),
    ) -> bool;
}

pub trait StaticVectorCommitment {
    type Domain;
    type Commitment;
    type BatchCommitment;

    fn setup(rng: &mut impl Rng, lambda: usize, n: usize) -> Self;

    fn commit(&mut self, m: &[Self::Domain]);

    fn open(&self, b: &Self::Domain, i: usize) -> Self::Commitment;

    fn verify(&self, b: &Self::Domain, i: usize, pi: &Self::Commitment) -> bool;

    fn batch_open(&self, b: &[Self::Domain], i: &[usize]) -> Self::BatchCommitment;

    fn batch_verify(&self, b: &[Self::Domain], i: &[usize], pi: &Self::BatchCommitment) -> bool;
}

pub trait DynamicVectorCommitment: StaticVectorCommitment {
    /// Changes the value at position `i`, from `b_prime`  to `b`.
    fn update(&mut self, b: &Self::Domain, b_prime: &Self::Domain, i: usize);
}


/// https://github.com/Chia-Network/vdf-competition/blob/master/classgroups.pdf
/// This trait abstracts the Group of unknown order that is used to sample our primes
/// RSA or Class groups of imaginary quadratic order
/// Binary quadratic forms 
/// class groups of binary quadratic forms omits the trusted setup that RSA needs.
/// The order of the class group of a negative prime discriminant d, where |d| ≡ 3 mod 4, 
/// is believed to be difficult to compute when |d| is sufficiently large, making the order 
/// of the class group effectively unknown. Therefore, a suitable discriminant — and its associated 
/// class group — can be chosen without the need for a trusted setup, which is a major advantage for 
/// using class groups in applications requiring groups of unknown order.
pub trait PrimeGroup {
    fn probably_prime(&self, iterations: u32) -> bool;
}