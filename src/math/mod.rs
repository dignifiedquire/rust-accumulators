#![cfg_attr(feature = "cargo-clippy", allow(clippy::many_single_char_names))]
use std::borrow::Cow;

use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use num_bigint::Sign::Plus;


pub mod prime;
pub mod prime_rand;

// use self::prime_rand::RandPrime;

// Few Functions taken from
// https://github.com/RustCrypto/RSA/blob/master/src/math.rs


/// Jacobi returns the Jacobi symbol (x/y), either +1, -1, or 0.
/// The y argument must be an odd integer.
pub fn jacobi(x: &BigInt, y: &BigInt) -> isize {
    if !y.is_odd() {
        panic!(
            "invalid arguments, y must be an odd integer,but got {:?}",
            y
        );
    }

    let mut a = x.clone();
    let mut b = y.clone();
    let mut j = 1;

    if b.is_negative() {
        if a.is_negative() {
            j = -1;
        }
        b = -b;
    }

    loop {
        if b.is_one() {
            return j;
        }
        if a.is_zero() {
            return 0;
        }

        a = a.mod_floor(&b);
        if a.is_zero() {
            return 0;
        }

        // a > 0

        // handle factors of 2 in a
        let s = a.trailing_zeros().unwrap();
        if s & 1 != 0 {
            let bmod8 = b.get_limb(0) & 7;
            if bmod8 == 3 || bmod8 == 5 {
                j = -j;
            }
        }

        let c = &a >> s; // a = 2^s*c

        // swap numerator and denominator
        if b.get_limb(0) & 3 == 3 && c.get_limb(0) & 3 == 3 {
            j = -j
        }

        a = b;
        b = c.clone();
    }
}



/// Generic trait to implement modular inverse
pub trait ModInverse<R: Sized>: Sized {
    /// Function to calculate the [modular multiplicative
    /// inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse) of an integer *a* modulo *m*.
    ///
    /// TODO: references
    /// Returns the modular inverse of `self`.
    /// If none exists it returns `None`.
    fn mod_inverse(self, m: R) -> Option<Self>;
}


/// Calculates the extended eucledian algorithm.
/// See https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm for details.
/// The returned values are
/// - greatest common divisor (1)
/// - Bezout coefficients (2)
// TODO: implement optimized variants
pub fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigInt, BigInt, BigInt) {
    let mut a = BigInt::from_biguint(Plus, a.clone());
    let mut b = BigInt::from_biguint(Plus, b.clone());

    let mut ua = BigInt::one();
    let mut va = BigInt::zero();

    let mut ub = BigInt::zero();
    let mut vb = BigInt::one();

    let mut q;
    let mut tmp;
    let mut r;

    while !b.is_zero() {
        q = &a / &b;
        r = &a % &b;

        a = b;
        b = r;

        tmp = ua;
        ua = ub.clone();
        ub = tmp - &q * &ub;

        tmp = va;
        va = vb.clone();
        vb = tmp - &q * &vb;
    }

    (a, ua, va)
}

impl<'a> ModInverse<&'a BigUint> for BigUint {
    fn mod_inverse(self, m: &'a BigUint) -> Option<BigUint> {
        match mod_inverse(
            Cow::Owned(BigInt::from_biguint(Plus, self)),
            &BigInt::from_biguint(Plus, m.clone()),
        ) {
            Some(res) => res.to_biguint(),
            None => None,
        }
    }
}

impl ModInverse<BigUint> for BigUint {
    fn mod_inverse(self, m: BigUint) -> Option<BigUint> {
        match mod_inverse(
            Cow::Owned(BigInt::from_biguint(Plus, self)),
            &BigInt::from_biguint(Plus, m),
        ) {
            Some(res) => res.to_biguint(),
            None => None,
        }
    }
}

impl<'a> ModInverse<&'a BigInt> for BigInt {
    fn mod_inverse(self, m: &'a BigInt) -> Option<BigInt> {
        mod_inverse(Cow::Owned(self), m)
    }
}

impl ModInverse<BigInt> for BigInt {
    fn mod_inverse(self, m: BigInt) -> Option<BigInt> {
        mod_inverse(Cow::Owned(self), &m)
    }
}

/// Calculate the modular inverse of `a`.
/// Implemenation is based on the naive version from wikipedia.
#[inline]
fn mod_inverse(g: Cow<BigInt>, n: &BigInt) -> Option<BigInt> {
    assert!(g.as_ref() != n, "g must not be equal to n");
    assert!(!n.is_negative(), "negative modulus not supported");

    let n = n.abs();
    let g = if g.is_negative() {
        g.mod_floor(&n).to_biguint().unwrap()
    } else {
        g.to_biguint().unwrap()
    };

    let (d, x, _) = extended_gcd(&g, &n.to_biguint().unwrap());

    if !d.is_one() {
        return None;
    }

    if x.is_negative() {
        Some(x + n)
    } else {
        Some(x)
    }
}



/// Calculates a = a.pow(b).
// TODO: this can be speed up using various techniques, like precomputations.
pub fn pow_assign(a: &mut BigUint, b: &BigUint) {
    if b.is_zero() {
        *a = BigUint::one();
    } else if b.is_odd() {
        let a_before = a.clone();
        pow_assign(a, &(b.clone() - 1u32));
        *a *= &a_before;
    } else {
        pow_assign(a, &(b.clone() / 2u32));
        *a *= a.clone();
    }
}

/// Calculates a ^ e % n.
pub fn modpow_uint_int(a: &BigUint, e: &BigInt, n: &BigUint) -> Option<BigUint> {
    match e.sign() {
        Sign::Plus => {
            // regular case
            Some(a.clone().modpow(&e.to_biguint().unwrap(), n))
        }
        Sign::Minus => {
            // exponent is negative, so we calculate the modular inverse of e.
            let a_signed = BigInt::from_biguint(Sign::Plus, a.clone());
            let n_signed = BigInt::from_biguint(Sign::Plus, n.clone());

            if let Some(a_inv) = a_signed.mod_inverse(&n_signed) {
                let e_abs = e.abs().to_biguint().unwrap();
                Some(a_inv.to_biguint().unwrap().modpow(&e_abs, n))
            } else {
                None
            }
        }
        Sign::NoSign => {
            // zero
            Some(BigUint::one())
        }
    }
}

/// Calculates the `(xy)`-th root of `g`, given the `x`-th root and `y`-th root of `g.`
/// Operations are `mod n`.
pub fn shamir_trick(
    root_x: &BigUint,
    root_y: &BigUint,
    x: &BigUint,
    y: &BigUint,
    n: &BigUint,
) -> Option<BigUint> {
    // Check that the roots match to the same element
    let g1 = root_x.modpow(x, n);
    let g2 = root_y.modpow(y, n);

    if g1 != g2 {
        return None;
    }

    // a, b <- Bezout(x, y)
    let (_, a, b) = extended_gcd(x, y);

    let l = modpow_uint_int(&root_x, &b, n);
    let r = modpow_uint_int(&root_y, &a, n);

    if let Some(l) = l {
        if let Some(r) = r {
            return Some((l * r).mod_floor(n));
        }
    }

    None
}

/// Given `y = g^x` and `x = \prod x_i`, calculates the `x_i`-th roots, for all `i`.
/// All operations are `mod n`.
pub fn root_factor(g: &BigUint, x: &[BigUint], n: &BigUint) -> Vec<BigUint> {
    let m = x.len();
    if m == 1 {
        return vec![g.clone()];
    }

    let m_prime = m.div_floor(&2);

    let (x_l, x_r) = x.split_at(m_prime);

    let g_l = {
        let mut p = BigUint::one();
        // the paper uses the upper part for g_L
        for x in x_r {
            p *= x;
        }

        g.modpow(&p, n)
    };

    let g_r = {
        let mut p = BigUint::one();
        // the paper uses the lower part for g_R
        for x in x_l {
            p *= x;
        }

        g.modpow(&p, n)
    };

    let mut res = root_factor(&g_l, x_l, n);
    res.extend(root_factor(&g_r, x_r, n));

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    use num_bigint::RandBigInt;
    use num_traits::{FromPrimitive, Pow};
    use rand::{thread_rng, Rng};
    use crate::math::prime_rand::RandPrime;

    #[test]
    fn test_pow_assign_basics() {
        for i in 0..1024 {
            for j in 0..128 {
                let res = BigUint::from_usize(i).unwrap().pow(j as u32);
                let mut res_big = BigUint::from_usize(i).unwrap();
                pow_assign(&mut res_big, &BigUint::from_usize(j).unwrap());
                assert_eq!(res_big, res);
            }
        }
    }

    #[test]
    fn test_quo_rem() {
        // Ref: https://www.wolframalpha.com/input/?i=QuotientRemainder%5B-10,+13%5D
        let (l, r) = &BigInt::from_i64(-10)
            .unwrap()
            .div_mod_floor(&BigInt::from_i64(13).unwrap());

        assert_eq!(
            (l, r),
            (
                &BigInt::from_i64(-1).unwrap(),
                &BigInt::from_i64(3).unwrap(),
            )
        );
    }

    #[test]
    fn test_modpow() {
        let cases = vec![["49", "-6193420858199668535", "2881", "6"]];

        for case in &cases {
            let a = BigUint::parse_bytes(case[0].as_bytes(), 10).unwrap();
            let e = BigInt::parse_bytes(case[1].as_bytes(), 10).unwrap();
            let n = BigUint::parse_bytes(case[2].as_bytes(), 10).unwrap();
            let expected = BigUint::parse_bytes(case[3].as_bytes(), 10).unwrap();

            let actual = modpow_uint_int(&a, &e, &n).unwrap();

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_root_factor() {
        let mut rng = thread_rng();

        for _ in 0..10 {
            let n = rng.gen_biguint(64);
            let g = rng.gen_biguint(64);
            let m: usize = rng.gen_range(1, 128);

            let x = (0..m).map(|_| rng.gen_biguint(64)).collect::<Vec<_>>();

            let r = root_factor(&g, &x, &n);

            let mut xs = BigUint::one();
            for e in &x {
                xs *= e;
            }
            let y = g.modpow(&xs, &n);

            for (root, x_i) in r.iter().zip(x.iter()) {
                // root is the x_i-th root of y
                // so we check that root^x_i = y
                assert_eq!(&root.clone().modpow(x_i, &n), &y);
            }
        }
    }

    #[test]
    fn test_shamir_trick() {
        let mut rng = thread_rng();

        for _ in 0..30 {
            let n = rng.gen_biguint(64);
            let g = rng.gen_prime(64);

            let x = rng.gen_prime(64);
            let y = rng.gen_prime(64);
            let z = rng.gen_prime(64);

            // the element we calc the root against
            let a = g.modpow(&(x.clone() * &y * &z), &n);
            let root_x = g.modpow(&(y.clone() * &z), &n);
            let root_y = g.modpow(&(x.clone() * &z), &n);

            // make sure they are actual roots
            assert_eq!(
                &root_x.modpow(&x, &n),
                &a,
                "root_x is not the x-th root of a"
            );
            assert_eq!(
                &root_y.modpow(&y, &n),
                &a,
                "root_y is not the y-th root of a"
            );

            let root = shamir_trick(&root_x, &root_y, &x, &y, &n).unwrap();

            // root is the xy-th root of a
            // so we check that root^xy = a
            assert_eq!(&root.clone().modpow(&(x.clone() * &y), &n), &a);
        }
    }

    #[test]
    fn test_extended_gcd_example() {
        // simple example for wikipedia
        let a = BigUint::from_u32(240).unwrap();
        let b = BigUint::from_u32(46).unwrap();
        let (q, s_k, t_k) = extended_gcd(&a, &b);

        assert_eq!(q, BigInt::from_i32(2).unwrap());
        assert_eq!(s_k, BigInt::from_i32(-9).unwrap());
        assert_eq!(t_k, BigInt::from_i32(47).unwrap());
    }

    #[test]
    fn test_extended_gcd_assumptions() {
        let mut rng = thread_rng();

        for i in 1..100 {
            let a = rng.gen_biguint(i * 128);
            let b = rng.gen_biguint(i * 128);
            let (q, s_k, t_k) = extended_gcd(&a, &b);

            let lhs = BigInt::from_biguint(Plus, a) * &s_k;
            let rhs = BigInt::from_biguint(Plus, b) * &t_k;
            assert_eq!(q, lhs + &rhs);
        }
    }

     #[test]
    fn test_jacobi() {
        let cases = [
            [0, 1, 1],
            [0, -1, 1],
            [1, 1, 1],
            [1, -1, 1],
            [0, 5, 0],
            [1, 5, 1],
            [2, 5, -1],
            [-2, 5, -1],
            [2, -5, -1],
            [-2, -5, 1],
            [3, 5, -1],
            [5, 5, 0],
            [-5, 5, 0],
            [6, 5, 1],
            [6, -5, 1],
            [-6, 5, 1],
            [-6, -5, -1],
        ];

        for case in cases.iter() {
            let x = BigInt::from_i64(case[0]).unwrap();
            let y = BigInt::from_i64(case[1]).unwrap();

            assert_eq!(case[2] as isize, jacobi(&x, &y), "jacobi({}, {})", x, y);
        }
    }


    #[test]
    fn test_mod_inverse() {
        let tests = [
            ["1234567", "458948883992"],
	    ["239487239847", "2410312426921032588552076022197566074856950548502459942654116941958108831682612228890093858261341614673227141477904012196503648957050582631942730706805009223062734745341073406696246014589361659774041027169249453200378729434170325843778659198143763193776859869524088940195577346119843545301547043747207749969763750084308926339295559968882457872412993810129130294592999947926365264059284647209730384947211681434464714438488520940127459844288859336526896320919633919"],
	    ["-10", "13"],
            ["-6193420858199668535", "2881"],
        ];

        for test in &tests {
            let element = BigInt::parse_bytes(test[0].as_bytes(), 10).unwrap();
            let modulus = BigInt::parse_bytes(test[1].as_bytes(), 10).unwrap();

            println!("{} modinv {}", element, modulus);
            let inverse = element.clone().mod_inverse(&modulus).unwrap();
            println!("inverse: {}", &inverse);
            let cmp = (inverse * &element).mod_floor(&modulus);

            assert_eq!(
                cmp,
                BigInt::one(),
                "mod_inverse({}, {}) * {} % {} = {}, not 1",
                &element,
                &modulus,
                &element,
                &modulus,
                &cmp
            );
        }

        // exhaustive tests for small numbers
        for n in 2..100 {
            let modulus = BigInt::from_u64(n).unwrap();
            for x in 1..n {
                for sign in vec![1i64, -1i64] {
                    let element = BigInt::from_i64(sign * x as i64).unwrap();
                    let gcd = element.gcd(&modulus);

                    if !gcd.is_one() {
                        continue;
                    }

                    let inverse = element.clone().mod_inverse(&modulus).unwrap();
                    let cmp = (&inverse * &element).mod_floor(&modulus);
                    println!("inverse: {}", &inverse);
                    assert_eq!(
                        cmp,
                        BigInt::one(),
                        "mod_inverse({}, {}) * {} % {} = {}, not 1",
                        &element,
                        &modulus,
                        &element,
                        &modulus,
                        &cmp
                    );
                }
            }
        }
    }
}
