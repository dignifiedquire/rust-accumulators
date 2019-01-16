//Because we want the accumulator to be secure without trusted setup,
//it must also be the case that in the underlying security assumptions
//the adversary can see the coins used while selecting the concrete module.

//In particular, an accumulator must have a public key divided into two parts,
//one of which (say, the RSA modulus n) is generated by using a public randomness known by the adversary,
//and another one (say, a generator of a large subgroup in Z∗n) can be chosen by using a non-public randomness.

//To use the class group of an imaginary quadratic order.
//One can easily generate an imaginary quadratic order by choosing a random discriminant,
//and when the discriminant is large enough, the order of the class group cannot be computed.

// (Class group setup). For a public setup where we do not want the private key to
// be known by anyone, one could choose G to be the class group of an imaginary quadratic
// field.

// / Binary quadratic forms
// / class groups of binary quadratic forms omits the trusted setup that RSA needs.
// / The order of the class group of a negative prime discriminant d, where |d| ≡ 3 mod 4,
// / is believed to be difficult to compute when |d| is sufficiently large, making the order
// / of the class group effectively unknown. Therefore, a suitable discriminant — and its associated
// / class group — can be chosen without the need for a trusted setup, which is a major advantage for
// / using class groups in applications requiring groups of unknown order.

use crate::traits::PrimeGroup;
use failure::{bail, Error};
use num_bigint::BigUint;
use rand::CryptoRng;
use rand::Rng;

use classygroup::{
    create_discriminant, gmp_classgroup::GmpClassGroup, BigNum, BigNumExt,
    ClassGroup as ClassyGroup,
};
use std::hash::Hash;

pub struct ClassGroup;

// If the discriminant is 1 mod 4 then a=2,b=1 can be used as a generator

// impl ClassyGroup {
//     fn generate_prime_from_seed<T: BigNumExt, V: ClassGroup<BigNum = T> + Eq + Hash>(
//         seed: &[u8],
//         int_size_bits: usize,
//     ) -> Result<(BigUint, BigUint), Error> {

//         //guaranteed to be a negative prime number
//         let discriminant = create_discriminant(&seed, int_size_bits);
//         //init with generators 1 & 2
//         let prime = V::from_ab_discriminant(2.into(), 1.into(), discriminant);

//         let generator = generator_for_discriminant(discriminant);

//         //TODO convert to BigUint
//         //BigUint::from_bytes_be(x);

//         return
//     }
// }