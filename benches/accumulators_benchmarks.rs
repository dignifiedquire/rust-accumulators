// Copyright 2018 Stichting Organism
//
// Copyright 2018 POA Networks Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


extern crate accumulators;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate rand;
extern crate rand_chacha;
extern crate classgroup;
#[macro_use] extern crate criterion;
extern crate blake2;



use criterion::Criterion;


//These benches are taken from various places that the subcomnets were brought into this crate

mod rsa_benches {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use accumulators::math::prime_rand::RandPrime;
    use accumulators::Accumulator;
    use accumulators::traits::{BatchedAccumulator, StaticAccumulator};
    use accumulators::group::RSAGroup;

    const N: usize = 3072;
    const L: usize = 256;

    
    fn bench_add_1(c: &mut Criterion) {
        let rng = &mut ChaChaRng::from_seed([0u8; 32]);

        let mut acc = Accumulator::setup::<RSAGroup, _>(rng, N);
        let x = rng.gen_prime(L);

        c.bench_function("bench_add_1", move |b| {
            b.iter(|| { acc.add(&x) })
        });  
    }

    fn bench_mem_wit_create_1(c: &mut Criterion) {
        let rng = &mut ChaChaRng::from_seed([0u8; 32]);

        let mut acc = Accumulator::setup::<RSAGroup, _>(rng, N);
        let x = rng.gen_prime(L);
        acc.add(&x);

        c.bench_function("bench_mem_wit_create_1", move |b| {
            b.iter(|| acc.mem_wit_create(&x))
        });
    }

    fn bench_ver_mem_1(c: &mut Criterion) {
        let rng = &mut ChaChaRng::from_seed([0u8; 32]);

        let mut acc = Accumulator::setup::<RSAGroup, _>(rng, N);
        let x = rng.gen_prime(L);
        acc.add(&x);
        let w = acc.mem_wit_create(&x);

        c.bench_function("bench_ver_mem_1", move |b| {
            b.iter(|| acc.ver_mem(&w, &x))
        });
    }

    fn bench_batch_add_1(c: &mut Criterion) {
        let rng = &mut ChaChaRng::from_seed([0u8; 32]);

        let mut acc = Accumulator::setup::<RSAGroup, _>(rng, N);
        let xs = vec![rng.gen_prime(L)];

        c.bench_function("bench_batch_add_1", move |b| {
            b.iter(|| acc.batch_add(&xs))
        });
    }

    fn bench_ver_batch_add_1(c: &mut Criterion) {
        let rng = &mut ChaChaRng::from_seed([0u8; 32]);

        let mut acc = Accumulator::setup::<RSAGroup, _>(rng, N);
        let xs = vec![rng.gen_prime(L)];
        let a_t = acc.state().clone();
        let w = acc.batch_add(&xs);

        c.bench_function("bench_ver_batch_add_1", move |b| {
            b.iter(|| acc.ver_batch_add(&w, &a_t, &xs))
        });
    }

    criterion_group!{
        name = rsa_benches;
        config = Criterion::default();
        targets =
            bench_add_1,
            bench_mem_wit_create_1,
            bench_batch_add_1,
            bench_ver_batch_add_1,
    }

}

mod prime_benches {
    use super::*;
    use num_bigint::BigUint;
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use accumulators::math::prime;
    use accumulators::math::prime_rand::RandPrime;

    const NUM: &'static str = "203956878356401977405765866929034577280193993314348263094772646453283062722701277632936616063144088173312372882677123879538709400158306567338328279154499698366071906766440037074217117805690872792848149112022286332144876183376326512083574821647933992961249917319836219304274280243803104015000563790123";


    fn probably_prime_0(c: &mut Criterion) {
        let x = BigUint::parse_bytes(NUM.as_bytes(), 10).unwrap();


        c.bench_function("probably_prime_0", move |b| {
             b.iter(|| prime::probably_prime(&x, 0))
        });
    }
 
    fn probably_prime_1(c: &mut Criterion) {
        let x = BigUint::parse_bytes(NUM.as_bytes(), 10).unwrap();


        c.bench_function("probably_prime_1", move |b| {
             b.iter(|| prime::probably_prime(&x, 1))
        });
    }
   
    fn probably_prime_5(c: &mut Criterion) {
        let x = BigUint::parse_bytes(NUM.as_bytes(), 10).unwrap();

        c.bench_function("probably_prime_5", move |b| {
             b.iter(|| prime::probably_prime(&x, 5))
        });
    }
  
    fn probably_prime_10(c: &mut Criterion) {
        let x = BigUint::parse_bytes(NUM.as_bytes(), 10).unwrap();

        c.bench_function("probably_prime_10", move |b| {
             b.iter(|| prime::probably_prime(&x, 10))
        });
    }
    
    fn probably_prime_20(c: &mut Criterion) {
        let x = BigUint::parse_bytes(NUM.as_bytes(), 10).unwrap();

        c.bench_function("probably_prime_20", move |b| {
             b.iter(|| prime::probably_prime(&x, 20))
        });
    }

    
    fn bench_prime_lucas(c: &mut Criterion) {
        let x = BigUint::parse_bytes(NUM.as_bytes(), 10).unwrap();

        c.bench_function("bench_prime_lucas", move |b| {
            b.iter(|| prime::probably_prime_lucas(&x))
        });
    }

    
    fn bench_prime_miller_rabin(c: &mut Criterion) {
    
        let x = BigUint::parse_bytes(NUM.as_bytes(), 10).unwrap();

        c.bench_function("bench_prime_miller_rabin", move |b| {
            b.iter(|| prime::probably_prime_miller_rabin(&x, 1, true))
        });
    }

    
    fn bench_gen_prime(c: &mut Criterion) {
        c.bench_function("bench_gen_prime", move |b| {
            let rng = &mut ChaChaRng::from_seed([0u8; 32]);
            b.iter(|| rng.gen_prime(1024))
        });
    }

    criterion_group!{
        name = prime_benches;
        config = Criterion::default();
        targets =
            probably_prime_0,
            probably_prime_1,
            probably_prime_5,
            probably_prime_10,
            probably_prime_20,
            bench_prime_lucas,
            bench_prime_miller_rabin,
            bench_gen_prime,
    }

}


mod classgroup_benches {
    use super::*;
    use classgroup::{gmp_classgroup::GmpClassGroup, ClassGroup};
    type Mpz = <GmpClassGroup as ClassGroup>::BigNum;
    use std::{cell::RefCell, rc::Rc};
    use accumulators::group::create_discriminant;
    use blake2::Blake2b;

    fn bench_square(c: &mut Criterion) {
        let bench_params = |c: &mut Criterion, len: u16, seed: &[u8]| {
            let i = Rc::new(RefCell::new(GmpClassGroup::generator_for_discriminant(
                create_discriminant::<Blake2b, Mpz>(seed, len),
            )));
            {
                let i = i.clone();
                c.bench_function(
                    &format!("square with seed {:?}: {}", seed, len),
                    move |b| b.iter(|| i.borrow_mut().square()),
                );
            }
            {
                let multiplier = i.borrow().clone();
                c.bench_function(
                    &format!("multiply with seed {:?}: {}", seed, len),
                    move |b| b.iter(|| *i.borrow_mut() *= &multiplier),
                );
            }
        };
        
        for &i in &[512, 1024, 2048] {
            bench_params(c, i, b"\xaa")
        }
    }

    criterion_group!{
        name = classgroup_benches;
        config = Criterion::default();
        targets =
            bench_square,
    }
}

criterion_main!(
    rsa_benches::rsa_benches,
    prime_benches::prime_benches,
    classgroup_benches::classgroup_benches
);

