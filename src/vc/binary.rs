use blake2::Blake2b;
use byteorder::{BigEndian, ByteOrder};
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use rand::Rng;

use crate::hash::hash_prime;
use crate::traits::*;

#[derive(Debug, Clone)]
pub struct BinaryVectorCommitment<A: UniversalAccumulator + BatchedAccumulator> {
    lambda: usize,
    n: usize,
    acc: A,
    pos: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Commitment {
    Mem(BigUint),
    NonMem((BigUint, BigInt)),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchCommitment(
    // membership proof
    (BigUint, BigUint),
    // non membership proof
    (BigUint, BigUint, (BigUint, BigUint, BigInt), BigUint),
);

impl<A: UniversalAccumulator + BatchedAccumulator> StaticVectorCommitment
    for BinaryVectorCommitment<A>
{
    type Domain = bool;
    type Commitment = Commitment;
    type BatchCommitment = BatchCommitment;

    fn setup(rng: &mut impl Rng, lambda: usize, n: usize) -> Self {
        BinaryVectorCommitment {
            lambda,
            n,
            acc: A::setup(rng, lambda),
            pos: 0,
        }
    }

    fn commit(&mut self, m: &[Self::Domain]) {
        let primes = m
            .iter()
            .enumerate()
            .filter(|(_, &m_i)| m_i)
            .map(|(i, _)| map_i_to_p_i(self.pos + i))
            .collect::<Vec<_>>();

        self.pos += m.len();
        self.acc.batch_add(&primes);
    }

    fn open(&self, b: &Self::Domain, i: usize) -> Self::Commitment {
        let p_i = map_i_to_p_i(i);

        if *b {
            Commitment::Mem(self.acc.mem_wit_create(&p_i))
        } else {
            let p = self.acc.non_mem_wit_create(&p_i);
            Commitment::NonMem(p)
        }
    }

    fn verify(&self, b: &Self::Domain, i: usize, pi: &Self::Commitment) -> bool {
        let p_i = map_i_to_p_i(i);

        if *b {
            match pi {
                Commitment::Mem(v) => self.acc.ver_mem(v, &p_i),
                Commitment::NonMem(_) => false,
            }
        } else {
            match pi {
                Commitment::Mem(_) => false,
                Commitment::NonMem(v) => self.acc.ver_non_mem(&v, &p_i),
            }
        }
    }

    fn batch_open(&self, b: &[Self::Domain], i: &[usize]) -> Self::BatchCommitment {
        assert_eq!(b.len(), i.len());

        let ones = b
            .iter()
            .enumerate()
            .filter(|(_, b_j)| **b_j)
            .map(|(j, _)| j);

        let zeros = b
            .iter()
            .enumerate()
            .filter(|(_, b_j)| !*b_j)
            .map(|(j, _)| j);

        let mut p_ones = BigUint::one();
        for j in ones {
            p_ones *= map_i_to_p_i(i[j]);
        }

        let pi_i = if p_ones.is_one() {
            (BigUint::zero(), BigUint::zero())
        } else {
            self.acc.mem_wit_create_star(&p_ones)
        };

        let mut p_zeros = BigUint::one();
        for j in zeros {
            p_zeros *= map_i_to_p_i(i[j]);
        }

        let pi_e = if p_zeros.is_one() {
            (
                BigUint::zero(),
                BigUint::zero(),
                (BigUint::zero(), BigUint::zero(), BigInt::zero()),
                BigUint::zero(),
            )
        } else {
            self.acc.non_mem_wit_create_star(&p_zeros)
        };

        BatchCommitment(pi_i, pi_e)
    }

    fn batch_verify(&self, b: &[Self::Domain], i: &[usize], pi: &Self::BatchCommitment) -> bool {
        assert_eq!(b.len(), i.len());

        let ones = b
            .iter()
            .enumerate()
            .filter(|(_, b_j)| **b_j)
            .map(|(j, _)| j);

        let mut p_ones = BigUint::one();
        for j in ones {
            p_ones *= map_i_to_p_i(i[j]);
        }

        if !p_ones.is_one() && !self.acc.ver_mem_star(&p_ones, &pi.0) {
            return false;
        }

        let zeros = b
            .iter()
            .enumerate()
            .filter(|(_, b_j)| !**b_j)
            .map(|(j, _)| j);

        let mut p_zeros = BigUint::one();
        for j in zeros {
            p_zeros *= map_i_to_p_i(i[j]);
        }

        if !p_zeros.is_one() && !self.acc.ver_non_mem_star(&p_zeros, &pi.1) {
            return false;
        }

        true
    }
}

impl<A: UniversalAccumulator + BatchedAccumulator> DynamicVectorCommitment
    for BinaryVectorCommitment<A>
{
    fn update(&mut self, b: &Self::Domain, b_prime: &Self::Domain, i: usize) {
        if b == b_prime {
            // Nothing to do
        } else if *b {
            self.acc.add(&map_i_to_p_i(i));
        } else {
            self.acc.del(&map_i_to_p_i(i)).expect("not a member");
        }
    }
}

fn map_i_to_p_i(i: usize) -> BigUint {
    let mut to_hash = [0u8; 8];
    BigEndian::write_u64(&mut to_hash, i as u64);
    hash_prime::<_, Blake2b>(&to_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::{Rng, SeedableRng, XorShiftRng};

    use crate::accumulator::Accumulator;
    use crate::primes::RSAGroup;

    #[test]
    fn test_binary_vc_basics() {
        let lambda = 128;
        let n = 1024;
        let mut rng = &mut XorShiftRng::from_seed([1u8; 16]);

        let mut vc = BinaryVectorCommitment::<Accumulator<RSAGroup>>::setup(&mut rng, lambda, n);

        let mut val: Vec<bool> = (0..64).map(|_| rng.gen()).collect();
        // set two bits manually, to make checks easier
        val[2] = true;
        val[3] = false;

        vc.commit(&val);

        // open a set bit
        let comm = vc.open(&true, 2);
        assert!(vc.verify(&true, 2, &comm), "invalid commitment (bit set)");

        // open a set bit
        let comm = vc.open(&false, 3);
        assert!(
            vc.verify(&false, 3, &comm),
            "invalid commitment (bit not set)"
        );
    }

    #[test]
    fn test_binary_vc_batch() {
        let lambda = 128;
        let n = 1024;
        let mut rng = &mut XorShiftRng::from_seed([1u8; 16]);

        let mut vc = BinaryVectorCommitment::<Accumulator>::setup(&mut rng, lambda, n);

        let val: Vec<bool> = (0..64).map(|_| rng.gen()).collect();
        vc.commit(&val);

        let committed = vec![val[2].clone(), val[3].clone(), val[9].clone()];
        let comm = vc.batch_open(&committed, &[2, 3, 9]);
        assert!(
            vc.batch_verify(&committed, &[2, 3, 9], &comm),
            "invalid commitment (bit set)"
        );
    }

    #[test]
    fn test_binary_vc_update() {
        let lambda = 128;
        let n = 1024;
        let mut rng = &mut XorShiftRng::from_seed([1u8; 16]);

        let mut vc = BinaryVectorCommitment::<Accumulator>::setup(&mut rng, lambda, n);

        let mut val: Vec<bool> = (0..64).map(|_| rng.gen()).collect();
        // set two bits manually, to make checks easier
        val[2] = true;
        val[3] = false;

        vc.commit(&val);

        let comm = vc.open(&true, 2);
        assert!(vc.verify(&true, 2, &comm), "invalid commitment (bit set)");

        vc.update(&false, &true, 2);

        // ensure old commitment fails now
        assert!(
            !vc.verify(&true, 2, &comm),
            "commitment should be invalid (bit set)"
        );

        let comm_new = vc.open(&false, 2);
        assert!(
            vc.verify(&false, 2, &comm_new),
            "invalid commitment (bit not set)"
        );
    }
}
