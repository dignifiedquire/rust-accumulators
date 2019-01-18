//! Key Value Map

//An accumulator is a commitment to a set. 
//A vector-commitment is a commitment to a positional vector. 
//We can use the vector-commitment to build a commitment to a key-value map.
//A key-value map is an associative data structure where elements from a key space K are mapped to a value space V ∪ {⊥}. 
//We say that a key k ∈ K is in the map if it does not map to ⊥. ⊥ is represented by 0.
//The key-space is represented by positions in the vector and the associated value is the data at the keys position.
//Note that if the key-space is large then we need a sparse vector

crate::vc::VectorCommitment;


/// A map based on a vector-commitment.
pub struct Map<K, V> A: UniversalAccumulator + BatchedAccumulator {
    vc: VectorCommitment<A>,
    //Value Store
    //values: 
}


impl Map {
    
    fn setup<G, R, A>(rng: &mut R, lambda: usize, n: usize) -> Self
    where
        G: PrimeGroup,
        R: CryptoRng + Rng,
        A: UniversalAccumulator + BatchedAccumulator
    {
        Map {
            vc: VectorCommitment::<A>::setup::<G, _>(rng, lambda, n);
        }
    }
}