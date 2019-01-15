use crate::traits::PrimeGroup;
use failure::{bail, Error};
use num_bigint::BigUint;
use rand::CryptoRng;
use rand::Rng;

use classgroup::{gmp_classgroup::GmpClassGroup, BigNum, BigNumExt, ClassGroup};
use create_discriminant::create_discriminant;

pub struct ClassGroup;

// If the discriminant is 1 mod 4 then a=2,b=1 can be used as a generator

impl PrimeGroup for ClassGroup {
    fn generate_primes<R: Rng + CryptoRng>(
        rng: &mut R,
        bit_size: usize,
    ) -> Result<(BigUint, BigUint), Error> {
        let mut entropy = [0u8; 32];
        csprng.fill_bytes(&mut entropy);

        //guaranteed to be a negative prime number
        let discriminant = create_discriminant(&entropy, int_size_bits);
        //
        let x = V::from_ab_discriminant(2.into(), 1.into(), discriminant);

        //TODO convert to BigUint
        BigUint::from_bytes_be(x);
    }
}
