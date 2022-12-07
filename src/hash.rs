use blake2::Digest;
use byteorder::{BigEndian, WriteBytesExt};
use generic_array::ArrayLength;
use num_bigint::prime::probably_prime;
use num_bigint::BigUint;
use num_integer::Integer;

// When the proofs are made non-interactive, using the
// Fiat-Shamir heuristic the challenge is generated by hashing the previous transcript

/// Hash the given numbers to a prime number.
/// Currently uses only 128bits.
pub fn hash_prime<O: ArrayLength<u8>, D: Digest<OutputSize = O>>(input: &[u8]) -> BigUint {
    let mut y = BigUint::from_bytes_be(&D::digest(input)[..16]);

    while !probably_prime(&y, 20) {
        y = BigUint::from_bytes_be(&D::digest(&y.to_bytes_be())[..16]);
    }

    y
}

/// Hash the given numbers into the given group.
/// Only works for `OutputSize >= |n|`.
pub fn hash_group<O: ArrayLength<u8>, D: Digest<OutputSize = O>>(
    input: &[u8],
    n: &BigUint,
) -> BigUint {
    let y = BigUint::from_bytes_be(&D::digest(input)[..]);

    y.mod_floor(n)
}

/// Nonce based Hash to prime
/// Prover provide a nonce such that H(nonce|| DATA ) = l with l ∈ Primes(λ).
/// Verification becomes a constant time operation which uses only a single primality check.
/// This of course allows an adversary to accumulate the same element twice but this can
/// be prevented by additionally hashing in the current state of the accumulator.
/// H(nonce || DATA || CURRENT_ROOT) = l with l ∈ Primes(λ).
/// We encode the given data as BIGENDIAN and then hash the concatination of these bytes.
/// We denote that hashsing the root is an optional parameter
pub fn nonce_hash<O: ArrayLength<u8>, D: Digest<OutputSize = O>>(
    nonce: u16,
    input: &[u8],
    root: Option<&BigUint>,
) -> Option<BigUint> {
    let mut vec = vec![];
    //nonce
    vec.write_u16::<BigEndian>(nonce).unwrap();
    //input
    vec.extend_from_slice(input);

    if let Some(root) = root {
        vec.append(&mut root.to_bytes_be());
    }

    let p = BigUint::from_bytes_be(&D::digest(vec.as_slice()));

    if probably_prime(&p, 20) {
        Some(p)
    } else {
        None
    }
}

///Verify if given BIGUINT is a prime, complements the nonce_hash() function
pub fn verify_nonce_hash(p: &BigUint) -> bool {
    probably_prime(p, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    use blake2::Blake2b512;
    use num_bigint::RandBigInt;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_hash_prime() {
        let mut rng = thread_rng();

        for i in 1..10 {
            let mut val = vec![0u8; i * 32];
            rng.fill(&mut val[..]);

            let h = hash_prime::<_, Blake2b512>(&val);
            assert!(probably_prime(&h, 20));
        }
    }

    #[test]
    fn test_hash_group() {
        let mut rng = thread_rng();

        for i in 1..10 {
            let mut val = vec![0u8; i * 32];
            rng.fill(&mut val[..]);
            let n = rng.gen_biguint(1024);

            let h = hash_group::<_, Blake2b512>(&val, &n);
            assert!(h <= n);
        }
    }

    #[test]
    fn test_hash_nonce() {
        let mut rng = thread_rng();
        let mut nonce: u16 = 0;
        for i in 1..10 {
            let mut val = vec![0u8; i * 32];
            rng.fill(&mut val[..]);
            let _n = rng.gen_biguint(1024);

            let mut h = nonce_hash::<_, Blake2b512>(nonce, &val, None);

            while h == None {
                nonce = nonce + 1;
                h = nonce_hash::<_, Blake2b512>(nonce, &val, None);
            }

            assert!(verify_nonce_hash(&h.unwrap()));
        }
    }
}
