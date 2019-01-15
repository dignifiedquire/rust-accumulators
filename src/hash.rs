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



use blake2::Digest;
use generic_array::ArrayLength;
use num_bigint::BigUint;
use num_integer::Integer;
use byteorder::{BigEndian, WriteBytesExt};
use num_bigint::prime::probably_prime;


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
pub fn nonce_hash<O: ArrayLength<u8>, D: Digest<OutputSize = O>>(nonce: u16, input: &[u8], root: Option<&BigUint>) -> Option<BigUint> { 
    let mut vec = vec![];
    //nonce
    vec.write_u16::<BigEndian>(nonce).unwrap();
    //input
    vec.extend_from_slice(input);

    if !root.is_none() {
        vec.append(&mut root.unwrap().to_bytes_be());    
    } 

    let p = BigUint::from_bytes_be(&D::digest(vec.as_slice()));
    
    if probably_prime(&p, 20) {
        return Some(p);
    } else {
        return None;
    }
   
}

///Verify if given BIGUINT is a prime, complements the nonce_hash() function
pub fn verify_nonce_hash(p: &BigUint) -> bool { 
    if probably_prime(p, 20) {
        return true;
    } else {
        return false;
    } 
}




#[cfg(test)]
mod tests {
    use super::*;

    use blake2::Blake2b;
    use num_bigint::RandBigInt;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_hash_prime() {
        let mut rng = thread_rng();

        for i in 1..10 {
            let mut val = vec![0u8; i * 32];
            rng.fill(&mut val[..]);

            let h = hash_prime::<_, Blake2b>(&val);
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

            let h = hash_group::<_, Blake2b>(&val, &n);
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
            let n = rng.gen_biguint(1024);

            let mut h = nonce_hash::<_, Blake2b>(nonce, &val, None);

            while h == None {
                nonce = nonce + 1;
                h = nonce_hash::<_, Blake2b>(nonce, &val, None);
            }

            assert!(verify_nonce_hash(&h.unwrap()));
        }
    }
}
