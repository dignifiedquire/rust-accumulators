// Copyright 2018 Stichting Organism
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

use crate::traits::PrimeGroup;
use failure::{bail, Error};
use rand::CryptoRng;
use rand::Rng;
use num_bigint::BigUint;

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