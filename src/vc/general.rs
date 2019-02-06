use bitvec;
use blake2::{Blake2b, Digest};
use num_bigint::BigUint;
use rand::rngs::OsRng;
use rand::{CryptoRng, Rng};
use rayon::prelude::*;

use crate::traits::*;
use crate::vc::BinaryVectorCommitment;

pub fn create_vector_commitment<A: UniversalAccumulator + BatchedAccumulator, G: PrimeGroup>(
    lambda: usize,
    n: usize,
) -> VectorCommitment<A> {
    let rng = &mut OsRng::new().expect("no secure randomness available");
    VectorCommitment::<A>::setup::<G, _>(rng, lambda, n)
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct VectorCommitment<A: UniversalAccumulator + BatchedAccumulator> {
    lambda: usize,
    n: usize,
    vc: BinaryVectorCommitment<A>,
}
impl<A: UniversalAccumulator + BatchedAccumulator> VectorCommitment<A> {
    pub fn commit_refs<'a>(&mut self, ms: impl IntoIterator<Item = &'a [u8]>) {
        let lambda = self.lambda;
        let comms = ms
            .into_iter()
            .map(|m| hash_binary(m, lambda).into_iter())
            .flatten();

        self.vc.commit(comms);
    }
}

impl<A: UniversalAccumulator + BatchedAccumulator> StaticVectorCommitment for VectorCommitment<A> {
    type Domain = Vec<u8>;
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
    fn commit(&mut self, ms: impl IntoIterator<Item = Self::Domain>) {
        let lambda = self.lambda;
        let comms = ms
            .into_iter()
            .map(|m| hash_binary(&m, lambda).into_iter())
            .flatten();

        self.vc.commit(comms);
    }

    fn open(&self, b: &Self::Domain, i: usize) -> Self::Commitment {
        let offset = i * self.lambda;

        let comm = hash_binary(b, self.lambda)
            .into_iter()
            .enumerate()
            .map(|(j, b)| (b, offset + j));

        self.vc.batch_open(comm)
    }

    fn verify(&self, b: &Self::Domain, i: usize, pi: &Self::Commitment) -> bool {
        let offset = i * self.lambda;

        let comm = hash_binary(b, self.lambda)
            .into_iter()
            .enumerate()
            .map(|(j, b)| (b, offset + j));

        self.vc.batch_verify(comm, pi)
    }

    fn batch_open(
        &self,
        b: impl IntoIterator<Item = (Self::Domain, usize)>,
    ) -> Self::BatchCommitment {
        let lambda = self.lambda;
        let comm = b
            .into_iter()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|(el, i)| {
                hash_binary(&el, lambda)
                    .into_iter()
                    .take(lambda)
                    .enumerate()
                    .map(|(j, b)| (b, i * lambda + j))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        self.vc.batch_open(comm)
    }

    fn batch_verify(
        &self,
        b: impl IntoIterator<Item = (Self::Domain, usize)>,
        pi: &Self::BatchCommitment,
    ) -> bool {
        let lambda = self.lambda;
        let comm = b
            .into_iter()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|(el, i)| {
                hash_binary(&el, lambda)
                    .into_iter()
                    .take(lambda)
                    .enumerate()
                    .map(|(j, b)| (b, i * lambda + j))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        self.vc.batch_verify(comm, pi)
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

fn hash_binary(bytes: &[u8], lambda: usize) -> bitvec::BitVec<bitvec::BigEndian, u8> {
    let mut res: Vec<u8> = Blake2b::digest(bytes)[..].to_vec();
    let byte_lambda = lambda / 8;
    // let mut res = bytes;
    res.resize(byte_lambda, 0u8);

    res.into()
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

        let val: Vec<_> = (0..3)
            .map(|_| rng.gen_biguint(lambda).to_bytes_be())
            .collect();
        vc.commit(val.clone());

        for i in 0..3 {
            let comm = vc.open(&val[i], i);
            assert!(vc.verify(&val[i], i, &comm), "invalid commitment {}", i);
        }
    }

    #[test]
    fn test_general_vc_batch() {
        let lambda = 256;
        let n = 1024;
        let rng = &mut ChaChaRng::from_seed([0u8; 32]);

        let mut vc = VectorCommitment::<Accumulator>::setup::<RSAGroup, _>(rng, lambda, n);

        let val: Vec<_> = (0..4)
            .map(|_| rng.gen_biguint(lambda).to_bytes_be())
            .collect();
        vc.commit(val.clone());

        let committed = vec![(val[1].clone(), 1), (val[3].clone(), 3)];
        let comm = vc.batch_open(committed.clone());
        assert!(vc.batch_verify(committed, &comm), "invalid commitment");
    }

    #[test]
    fn test_general_vc_update() {
        let lambda = 128;
        let n = 1024;
        let rng = &mut ChaChaRng::from_seed([0u8; 32]);

        let mut vc = VectorCommitment::<Accumulator>::setup::<RSAGroup, _>(rng, lambda, n);
        let val: Vec<_> = (0..4)
            .map(|_| rng.gen_biguint(lambda).to_bytes_be())
            .collect();

        vc.commit(val.clone());

        let comm = vc.open(&val[2], 2);
        assert!(vc.verify(&val[2], 2, &comm), "invalid commitment");

        let new_val = rng.gen_biguint(128).to_bytes_be();
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
