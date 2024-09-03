use core::f64::consts::E;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lambert_w::{
    lambert_w0, lambert_w0f, lambert_wm1, lambert_wm1f, sp_lambert_w0, sp_lambert_wm1,
};
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    rngs::SmallRng,
    Rng, SeedableRng,
};
use std::{
    ops::RangeBounds,
    time::{Duration, Instant},
};

const E32: f32 = core::f32::consts::E;

fn init_vec_of_rng_and_bench<R, T, F>(f: F, range: R, iters: u64, rng: &mut SmallRng) -> Duration
where
    R: RangeBounds<T> + Clone + SampleRange<T>,
    T: SampleUniform + Copy + PartialOrd,
    F: Fn(T) -> T,
{
    let datas: Vec<T> = (0..iters).map(|_| rng.gen_range(range.clone())).collect();
    let start = Instant::now();
    datas.iter().for_each(|&z| {
        black_box(f(z));
    });
    let duration = start.elapsed();
    drop(datas);
    duration
}

fn random_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("random inputs");
    let mut rng = SmallRng::seed_from_u64(0b1010101010101);
    group.bench_function("W_0 50 bits", |b| {
        b.iter_custom(|iters| {
            init_vec_of_rng_and_bench(lambert_w0, -1.0 / E..f64::MAX, iters, &mut rng)
        })
    });
    group.bench_function("W_0 24 bits", |b| {
        b.iter_custom(|iters| {
            init_vec_of_rng_and_bench(sp_lambert_w0, -1.0 / E..f64::MAX, iters, &mut rng)
        })
    });
    group.bench_function("W_0 24 bits on f32", |b| {
        b.iter_custom(|iters| {
            init_vec_of_rng_and_bench(lambert_w0f, -1.0 / E32..f32::MAX, iters, &mut rng)
        })
    });
    group.bench_function("W_-1 50 bits", |b| {
        b.iter_custom(|iters| {
            init_vec_of_rng_and_bench(lambert_wm1, -1.0 / E..=0.0, iters, &mut rng)
        })
    });
    group.bench_function("W_-1 24 bits", |b| {
        b.iter_custom(|iters| {
            init_vec_of_rng_and_bench(sp_lambert_wm1, -1.0 / E..=0.0, iters, &mut rng)
        })
    });
    group.bench_function("W_-1 24 bits on f32", |b| {
        b.iter_custom(|iters| {
            init_vec_of_rng_and_bench(lambert_wm1f, -1.0 / E32..=0.0, iters, &mut rng)
        })
    });
}

criterion_group!(benches, random_benches);
criterion_main!(benches);
