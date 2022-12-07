use bitvec::prelude::*;
use blake2::{Blake2b512, Digest};
use num_bigint::BigUint;
use rand::rngs::OsRng;
use rand::{CryptoRng, Rng};

use crate::traits::*;
use crate::vc::BinaryVectorCommitment;

pub fn create_vector_commitment<A: UniversalAccumulator + BatchedAccumulator, G: PrimeGroup>(
    lambda: usize,
    n: usize,
) -> VectorCommitment<A> {
    let mut rng = OsRng;
    VectorCommitment::<A>::setup::<G, _>(&mut rng, lambda, n)
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct VectorCommitment<A: UniversalAccumulator + BatchedAccumulator> {
    lambda: usize,
    n: usize,
    vc: BinaryVectorCommitment<A>,
}

impl<A: UniversalAccumulator + BatchedAccumulator> StaticVectorCommitment for VectorCommitment<A> {
    type Domain = BigUint;
    type Commitment = <BinaryVectorCommitment<A> as StaticVectorCommitment>::BatchCommitment;
    type BatchCommitment = <BinaryVectorCommitment<A> as StaticVectorCommitment>::BatchCommitment;

    fn setup<G, R>(rng: &mut R, lambda: usize, n: usize) -> Self
    where
        G: PrimeGroup,
        R: CryptoRng + Rng,
    {
        VectorCommitment {
            lambda,
            n,
            vc: BinaryVectorCommitment::<A>::setup::<G, _>(rng, lambda, n),
        }
    }

    // Internally we map the incoming integers onto a binary vc in the following way
    // ms: [a, b, c]
    // a' = hash_binary(a), b' ..
    // vc[a'..., b'..., c'...]
    fn commit(&mut self, ms: &[Self::Domain]) {
        for m in ms {
            let comm = hash_binary(m, self.lambda).into_iter().collect::<Vec<_>>();
            debug_assert!(comm.len() == self.lambda);
            self.vc.commit(&comm);
        }
    }

    fn open(&self, b: &Self::Domain, i: usize) -> Self::Commitment {
        let comm = hash_binary(b, self.lambda).into_iter().collect::<Vec<_>>();
        let offset = i * self.lambda;
        let is = (0..comm.len()).map(|j| offset + j).collect::<Vec<_>>();

        self.vc.batch_open(&comm, &is)
    }

    fn verify(&self, b: &Self::Domain, i: usize, pi: &Self::Commitment) -> bool {
        let comm = hash_binary(b, self.lambda).into_iter().collect::<Vec<_>>();
        let offset = i * self.lambda;
        let is = (0..comm.len()).map(|j| offset + j).collect::<Vec<_>>();

        self.vc.batch_verify(&comm, &is, pi)
    }

    fn batch_open(&self, b: &[Self::Domain], is: &[usize]) -> Self::BatchCommitment {
        debug_assert!(b.len() == is.len());

        let mut comm = Vec::with_capacity(self.lambda * b.len());
        let mut comm_is = Vec::with_capacity(self.lambda * is.len());

        for (el, i) in b.iter().zip(is) {
            let c = hash_binary(el, self.lambda).into_iter().collect::<Vec<_>>();
            comm.extend(&c);
            let offset = i * self.lambda;
            comm_is.extend((0..c.len()).map(|j| offset + j));
        }

        self.vc.batch_open(&comm, &comm_is)
    }

    fn batch_verify(&self, b: &[Self::Domain], is: &[usize], pi: &Self::BatchCommitment) -> bool {
        debug_assert!(b.len() == is.len());

        let mut comm = Vec::with_capacity(self.lambda * b.len());
        let mut comm_is = Vec::with_capacity(self.lambda * is.len());

        for (el, i) in b.iter().zip(is) {
            let c = hash_binary(el, self.lambda).into_iter().collect::<Vec<_>>();
            comm.extend(&c);
            let offset = i * self.lambda;
            comm_is.extend((0..c.len()).map(|j| offset + j));
        }

        self.vc.batch_verify(&comm, &comm_is, pi)
    }

    fn state(&self) -> &BigUint {
        self.vc.state()
    }
}

impl<A: UniversalAccumulator + BatchedAccumulator> DynamicVectorCommitment for VectorCommitment<A> {
    fn update(&mut self, b: &Self::Domain, b_prime: &Self::Domain, i: usize) {
        if b == b_prime {
            // Nothing to do
        } else {
            let comm = hash_binary(b, self.lambda).into_iter();
            let comm_prime = hash_binary(b_prime, self.lambda).into_iter();
            let offset = i * self.lambda;
            let is = (0..self.lambda).map(|j| offset + j);

            // This is updating bit by bit, but only those bits that actually changed require work.
            for (el, (el_prime, i)) in comm.zip(comm_prime.zip(is)) {
                self.vc.update(&el, &el_prime, i);
            }
        }
    }
}

fn hash_binary(m: &BigUint, lambda: usize) -> BitVec<u8, Msb0> {
    let bytes = Blake2b512::digest(&m.to_bytes_be());
    let len = std::cmp::min(bytes.len(), lambda / 8);

    BitVec::from_slice(&bytes[..len])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accumulator::Accumulator;
    use crate::group::RSAGroup;
    use num_bigint::RandBigInt;
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;

    #[test]
    fn test_general_vc_basics() {
        let lambda = 128;
        let n = 1024;
        let rng = &mut ChaChaRng::from_seed([0u8; 32]);

        let mut vc = VectorCommitment::<Accumulator>::setup::<RSAGroup, _>(rng, lambda, n);

        let val: Vec<BigUint> = (0..3).map(|_| rng.gen_biguint(16)).collect();
        vc.commit(&val);

        for i in 0..3 {
            let comm = vc.open(&val[i], i);
            assert!(vc.verify(&val[i], i, &comm), "invalid commitment {}", i);
        }
    }

    #[test]
    fn test_general_vc_batch() {
        let lambda = 128;
        let n = 1024;
        let rng = &mut ChaChaRng::from_seed([0u8; 32]);

        let mut vc = VectorCommitment::<Accumulator>::setup::<RSAGroup, _>(rng, lambda, n);

        let val: Vec<BigUint> = (0..4).map(|_| rng.gen_biguint(32)).collect();
        vc.commit(&val);

        let committed = vec![val[1].clone(), val[3].clone()];
        let comm = vc.batch_open(&committed, &[1, 3]);
        assert!(
            vc.batch_verify(&committed, &[1, 3], &comm),
            "invalid commitment"
        );
    }

    #[test]
    fn test_general_vc_update() {
        let lambda = 128;
        let n = 1024;
        let rng = &mut ChaChaRng::from_seed([0u8; 32]);

        let mut vc = VectorCommitment::<Accumulator>::setup::<RSAGroup, _>(rng, lambda, n);
        let val: Vec<BigUint> = (0..4).map(|_| rng.gen_biguint(32)).collect();

        vc.commit(&val);

        let comm = vc.open(&val[2], 2);
        assert!(vc.verify(&val[2], 2, &comm), "invalid commitment");

        let new_val = rng.gen_biguint(128);
        vc.update(&new_val, &val[2], 2);

        // ensure old commitment fails now
        assert!(
            !vc.verify(&new_val, 2, &comm),
            "commitment should be invalid"
        );

        let comm_new = vc.open(&new_val, 2);
        assert!(vc.verify(&new_val, 2, &comm_new), "invalid commitment");
    }
}
