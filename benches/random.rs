use core::f64::consts::E;
use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use lambert_w::{lambert_w_0, lambert_w_m1, sp_lambert_w_0, sp_lambert_w_m1};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::time::Instant;

fn random_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("random inputs");
    let mut rng = Pcg32::seed_from_u64(0);
    group.bench_function("W_0 50 bits", |b| {
        b.iter_custom(|iters| {
            let datas: Vec<f64> = (0..iters)
                .map(|_| rng.gen_range(-1.0 / E..f64::MAX))
                .collect();
            let start = Instant::now();
            for &z in &datas {
                black_box(lambert_w_0(z));
            }
            start.elapsed()
        })
    });
    group.bench_function("W_0 24 bits", |b| {
        b.iter_custom(|iters| {
            let datas: Vec<f64> = (0..iters)
                .map(|_| rng.gen_range(-1.0 / E..f64::MAX))
                .collect();
            let start = Instant::now();
            for &z in &datas {
                black_box(sp_lambert_w_0(z));
            }
            start.elapsed()
        })
    });
    group.bench_function("W_-1 50 bits", |b| {
        b.iter_custom(|iters| {
            let datas: Vec<f64> = (0..iters).map(|_| rng.gen_range(-1.0 / E..=0.0)).collect();
            let start = Instant::now();
            for &z in &datas {
                black_box(lambert_w_m1(z));
            }
            start.elapsed()
        })
    });
    group.bench_function("W_-1 24 bits", |b| {
        b.iter_custom(|iters| {
            let datas: Vec<f64> = (0..iters).map(|_| rng.gen_range(-1.0 / E..=0.0)).collect();
            let start = Instant::now();
            for &z in &datas {
                black_box(sp_lambert_w_m1(z));
            }
            start.elapsed()
        })
    });
}

criterion_group!(benches, random_benches);
criterion_main!(benches);
