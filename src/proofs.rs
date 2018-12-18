use blake2::{Blake2b, Digest};
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;

use crate::hash::{hash_group, hash_prime};
use crate::math::modpow_uint_int;

/// NI-PoE Prove
/// Assumes `u^x = w`
/// All operations are `mod n`.
pub fn ni_poe_prove(x: &BigUint, u: &BigUint, w: &BigUint, n: &BigUint) -> BigUint {
    debug_assert!(&u.modpow(x, n) == w, "invalid input");

    // l <- H_prime(x, u, w)
    let mut to_hash = x.to_bytes_be();
    to_hash.extend(&u.to_bytes_be());
    to_hash.extend(&w.to_bytes_be());

    let l = hash_prime::<_, Blake2b>(&to_hash);

    // q <- floor(x/l)
    let q = x.div_floor(&l);

    // Q <- u^q
    u.modpow(&q, n)
}

/// NI-PoE Verify
/// Assumes `u^x = w`
/// All operations are `mod n`.
pub fn ni_poe_verify(x: &BigUint, u: &BigUint, w: &BigUint, q: &BigUint, n: &BigUint) -> bool {
    // l <- H_prime(x, u, w)
    let mut to_hash = x.to_bytes_be();
    to_hash.extend(&u.to_bytes_be());
    to_hash.extend(&w.to_bytes_be());

    let l = hash_prime::<_, Blake2b>(&to_hash);

    // r <- x mod l
    let r = x.mod_floor(&l);

    // Q^l u^r == w
    &((q.modpow(&l, &n) * &u.modpow(&r, &n)) % n) == w
}

/// NI-PoKE2 Prove
/// assumes `u^x = w`
/// All operations are `mod n`.
pub fn ni_poke2_prove(
    x: impl Into<BigInt>,
    u: &BigUint,
    w: &BigUint,
    n: &BigUint,
) -> (BigUint, BigUint, BigInt) {
    let x: BigInt = x.into();
    debug_assert!(&modpow_uint_int(u, &x, n).unwrap() == w, "invalid input");

    // g <- H_G(u, w)
    let mut to_hash = u.to_bytes_be();
    to_hash.extend(&w.to_bytes_be());
    let g = hash_group::<_, Blake2b>(&to_hash, n);

    // z = g^x
    let z = modpow_uint_int(&g, &x, n).expect("invalid state");

    // l <- H_prime(u, w, z)
    to_hash.extend(&z.to_bytes_be());
    let l: BigInt = hash_prime::<_, Blake2b>(&to_hash).into();

    // alpha = H(u, w, z, l)
    to_hash.extend(&l.to_bytes_be().1);
    let alpha = BigUint::from_bytes_be(&Blake2b::digest(&to_hash)[..]);

    // q <- floor(x/l)
    // r <- x % l
    let (q, r) = x.div_rem(&l);

    // Q <- (ug^alpha)^q
    let q_big = modpow_uint_int(&(u * &g.modpow(&alpha, n)), &q, n).expect("invalid state");

    (z, q_big, r)
}

/// NI-PoKE2 Verify
/// assumes `u^x = w`
/// All operations are `mod n`
pub fn ni_poke2_verify(
    u: &BigUint,
    w: &BigUint,
    pi: &(BigUint, BigUint, BigInt),
    n: &BigUint,
) -> bool {
    // {z, Q, r} <- pi
    let (z, q_big, r) = pi;

    // g <- H_G(u, w)
    let mut to_hash = u.to_bytes_be();
    to_hash.extend(&w.to_bytes_be());
    let g = hash_group::<_, Blake2b>(&to_hash, n);

    // l <- H_prime(u, w, z)
    to_hash.extend(&z.to_bytes_be());
    let l = hash_prime::<_, Blake2b>(&to_hash);

    // alpha = H(u, w, z, l)
    to_hash.extend(&l.to_bytes_be());
    let alpha = BigUint::from_bytes_be(&Blake2b::digest(&to_hash)[..]);

    // Q^l(ug^alpha)^r
    let lhs: BigInt = ((q_big.modpow(&l, n)
        * modpow_uint_int(&(u * &g.modpow(&alpha, n)), &r, n).expect("invalid state"))
        % n)
        .into();

    // wz^alpha
    let z_alpha = z.modpow(&alpha, n);
    let rhs: BigInt = ((w * z_alpha) % n).into();

    lhs == rhs
}

#[cfg(test)]
mod tests {
    use super::*;

    use num_bigint::RandBigInt;
    use num_traits::One;
    use rand::thread_rng;
    use rsa::RandPrime;

    #[test]
    fn test_ni_poe() {
        let mut rng = thread_rng();
        for _ in 1..4 {
            for j in 1..10 {
                for k in 1..4 {
                    let p = rng.gen_prime(128);
                    let q = rng.gen_prime(128);
                    let n = p * q;

                    let mut x = BigUint::one();
                    for _ in 0..j {
                        x *= rng.gen_prime(256);
                    }
                    let u = rng.gen_biguint(k * 64);
                    let w = u.modpow(&x, &n);

                    let q = ni_poe_prove(&x, &u, &w, &n);
                    assert!(ni_poe_verify(&x, &u, &w, &q, &n))
                }
            }
        }
    }

    #[test]
    fn test_ni_poke2() {
        let mut rng = thread_rng();

        for i in 1..4 {
            for j in 1..4 {
                for k in 1..4 {
                    let n = rng.gen_biguint(i * 64);

                    let x = rng.gen_prime(j * 128);
                    let u = rng.gen_prime(k * 64);
                    let w = u.modpow(&x, &n);

                    let pi = ni_poke2_prove(x.clone(), &u, &w, &n);
                    assert!(ni_poke2_verify(&u, &w, &pi, &n))
                }
            }
        }
    }
}
