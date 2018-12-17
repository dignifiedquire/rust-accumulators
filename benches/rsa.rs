#![feature(test)]

extern crate accumulators;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate rand;
extern crate rsa;
extern crate test;

use num_bigint::Sign;
use num_integer::Integer;
use num_traits::FromPrimitive;
use rand::{SeedableRng, XorShiftRng};
use rsa::RandPrime;
use test::{black_box, Bencher};

use accumulators::rsa::RsaAccumulator;
use accumulators::traits::{BatchedAccumulator, StaticAccumulator};

const N: usize = 3072;
const L: usize = 256;

#[bench]
fn bench_add_1(b: &mut Bencher) {
    let rng = &mut XorShiftRng::from_seed([1u8; 16]);

    let mut acc = RsaAccumulator::setup(N);
    let x = rng.gen_prime(L);

    b.iter(|| {
        acc.add(&x);
    })
}

#[bench]
fn bench_mem_wit_create_1(b: &mut Bencher) {
    let rng = &mut XorShiftRng::from_seed([1u8; 16]);

    let mut acc = RsaAccumulator::setup(N);
    let x = rng.gen_prime(L);
    acc.add(&x);

    b.iter(|| black_box(acc.mem_wit_create(&x)))
}

#[bench]
fn bench_ver_mem_1(b: &mut Bencher) {
    let rng = &mut XorShiftRng::from_seed([1u8; 16]);

    let mut acc = RsaAccumulator::setup(N);
    let x = rng.gen_prime(L);
    acc.add(&x);
    let w = acc.mem_wit_create(&x);

    b.iter(|| {
        black_box(acc.ver_mem(&w, &x));
    })
}

#[bench]
fn bench_batch_add_1(b: &mut Bencher) {
    let rng = &mut XorShiftRng::from_seed([1u8; 16]);

    let mut acc = RsaAccumulator::setup(N);
    let xs = vec![rng.gen_prime(L)];

    b.iter(|| {
        black_box(acc.batch_add(&xs));
    })
}

#[bench]
fn bench_ver_batch_add_1(b: &mut Bencher) {
    let rng = &mut XorShiftRng::from_seed([1u8; 16]);

    let mut acc = RsaAccumulator::setup(N);
    let xs = vec![rng.gen_prime(L)];
    let a_t = acc.state().clone();
    let w = acc.batch_add(&xs);

    b.iter(|| {
        black_box(acc.ver_batch_add(&w, &a_t, &xs));
    })
}
