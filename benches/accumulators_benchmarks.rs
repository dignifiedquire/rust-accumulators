

extern crate accumulators;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate rand;
#[macro_use] extern crate criterion;



use criterion::Criterion;


mod rsa_benches {
    use super::*;
    use num_bigint::Sign;
    use num_integer::Integer;
    use num_traits::FromPrimitive;
    use rand::{SeedableRng, XorShiftRng};
    use accumulators::math::prime_rand::RandPrime;
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

mod prime_benches {
    use super::*;
    use num_bigint::BigUint;
    use rand::{SeedableRng, StdRng};
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
        let mut rng = StdRng::from_seed([1u8; 32]);

        c.bench_function("bench_gen_prime", move |b| {
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

criterion_main!(
    rsa_benches::rsa_benches,
    prime_benches::prime_benches
);

