use crate::traits::PrimeGroup;
use failure::{bail, Error};
use num_bigint::BigUint;
use rand::CryptoRng;
use rand::Rng;

use classgroup::{gmp_classgroup::GmpClassGroup, BigNum, BigNumExt, ClassGroup};
use super::create_discriminant::create_discriminant;
use std::hash::Hash;



pub struct ClassyGroup;

// If the discriminant is 1 mod 4 then a=2,b=1 can be used as a generator

impl ClassyGroup {
    fn generate_prime_from_seed<T: BigNumExt, V: ClassGroup<BigNum = T> + Eq + Hash>(
        seed: &[u8],
        int_size_bits: usize,
    ) -> Result<(BigUint, BigUint), Error> {
       

        //guaranteed to be a negative prime number
        let discriminant = create_discriminant(&seed, int_size_bits);
        //init with generators 1 & 2
        let prime = V::from_ab_discriminant(2.into(), 1.into(), discriminant);

        let generator = generator_for_discriminant(discriminant);

        //TODO convert to BigUint
        //BigUint::from_bytes_be(x);

        return 
    }
}
