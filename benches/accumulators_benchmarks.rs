

extern crate accumulators;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate rand;
extern crate rsa;
#[macro_use] extern crate criterion;



use criterion::Criterion;


mod rsa_benches {
    use super::*;
    use num_bigint::Sign;
    use num_integer::Integer;
    use num_traits::FromPrimitive;
    use rand::{SeedableRng, XorShiftRng};
    use rsa::RandPrime;
    use accumulators::rsa::RsaAccumulator;
    use accumulators::traits::{BatchedAccumulator, StaticAccumulator};


    const N: usize = 3072;
    const L: usize = 256;

    
    fn bench_add_1(c: &mut Criterion) {
        let rng = &mut XorShiftRng::from_seed([1u8; 16]);

        let mut acc = RsaAccumulator::setup(rng, N);
        let x = rng.gen_prime(L);

        c.bench_function("bench_add_1", move |b| {
            b.iter(|| { acc.add(&x) })
        });

       
    }

    fn bench_mem_wit_create_1(c: &mut Criterion) {
        let rng = &mut XorShiftRng::from_seed([1u8; 16]);

        let mut acc = RsaAccumulator::setup(rng, N);
        let x = rng.gen_prime(L);
        acc.add(&x);

        c.bench_function("bench_mem_wit_create_1", move |b| {
            b.iter(|| acc.mem_wit_create(&x))
        });
    }

    fn bench_ver_mem_1(c: &mut Criterion) {
        let rng = &mut XorShiftRng::from_seed([1u8; 16]);

        let mut acc = RsaAccumulator::setup(rng, N);
        let x = rng.gen_prime(L);
        acc.add(&x);
        let w = acc.mem_wit_create(&x);

        c.bench_function("bench_ver_mem_1", move |b| {
            b.iter(|| acc.ver_mem(&w, &x))
        });
    }

    fn bench_batch_add_1(c: &mut Criterion) {
        let rng = &mut XorShiftRng::from_seed([1u8; 16]);

        let mut acc = RsaAccumulator::setup(rng, N);
        let xs = vec![rng.gen_prime(L)];

        c.bench_function("bench_batch_add_1", move |b| {
            b.iter(|| acc.batch_add(&xs))
        });
    }

    fn bench_ver_batch_add_1(c: &mut Criterion) {
        let rng = &mut XorShiftRng::from_seed([1u8; 16]);

        let mut acc = RsaAccumulator::setup(rng, N);
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

criterion_main!(
    rsa_benches::rsa_benches,
);

