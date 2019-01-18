use crate::traits::PrimeGroup;
use failure::{bail, Error};
use num_bigint::traits::ModInverse;
use num_bigint::{BigUint, RandPrime};
use num_traits::{FromPrimitive, One, Zero};
use rand::CryptoRng;
use rand::Rng;

pub struct RSAGroup;

impl PrimeGroup for RSAGroup {
    // Based on https://github.com/RustCrypto/RSA/blob/master/src/algorithms.rs
    fn generate_primes<R: Rng + CryptoRng>(
        rng: &mut R,
        bit_size: usize,
    ) -> Result<(BigUint, BigUint), Error> {
        // Default exponent for RSA keys.
        const EXP: u64 = 65547;

        if bit_size < 64 {
            bail!("too few bits");
        }

        let nprimes = 2;
        let mut primes = vec![BigUint::zero(); nprimes];
        let n_final: BigUint;
        // let d_final: BigUint;

        'next: loop {
            let mut todo = bit_size;
            // `gen_prime` should set the top two bits in each prime.
            // Thus each prime has the form
            //   p_i = 2^bitlen(p_i) × 0.11... (in base 2).
            // And the product is:
            //   P = 2^todo × α
            // where α is the product of nprimes numbers of the form 0.11...
            //
            // If α < 1/2 (which can happen for nprimes > 2), we need to
            // shift todo to compensate for lost bits: the mean value of 0.11...
            // is 7/8, so todo + shift - nprimes * log2(7/8) ~= bits - 1/2
            // will give good results.
            if nprimes >= 7 {
                todo += (nprimes - 2) / 5;
            }

            for (i, prime) in primes.iter_mut().enumerate() {
                *prime = rng.gen_prime(todo / (nprimes - i));
                todo -= prime.bits();
            }

            // Makes sure that primes is pairwise unequal.
            for (i, prime1) in primes.iter().enumerate() {
                for prime2 in primes.iter().take(i) {
                    if prime1 == prime2 {
                        continue 'next;
                    }
                }
            }

            let mut n = BigUint::one();
            let mut totient = BigUint::one();

            for prime in &primes {
                n *= prime;
                totient *= prime - BigUint::one();
            }

            if n.bits() != bit_size {
                // This should never happen for nprimes == 2 because
                // gen_prime should set the top two bits in each prime.
                // For nprimes > 2 we hope it does not happen often.
                continue 'next;
            }

            let exp = BigUint::from_u64(EXP).expect("invalid static exponent");
            if let Some(_d) = exp.mod_inverse(totient) {
                n_final = n;
                // d_final = d;
                break;
            }
        }

        // This is a trusted setup, as we do know `p` and `q`, even though
        // we choose not to store them.
        let _q = primes.pop().unwrap();
        let _p = primes.pop().unwrap();

        Ok((
            n_final,
            BigUint::from_u64(EXP).expect("invalid static exponent"),
        ))
    }
}
