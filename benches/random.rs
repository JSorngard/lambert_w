use core::f64::consts::E;
use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use lambert_w::{
    lambert_w0, lambert_w0f, lambert_wm1, lambert_wm1f, sp_lambert_w0, sp_lambert_wm1,
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::time::Instant;

const E32: f32 = E as f32;

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
                black_box(lambert_w0(z));
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
                black_box(sp_lambert_w0(z));
            }
            start.elapsed()
        })
    });
    group.bench_function("W_0 24 bits on f32", |b| {
        b.iter_custom(|iters| {
            let datas: Vec<f32> = (0..iters)
                .map(|_| rng.gen_range(-1.0 / E32..f32::MAX))
                .collect();
            let start = Instant::now();
            for &z in &datas {
                black_box(lambert_w0f(z));
            }
            start.elapsed()
        })
    });
    group.bench_function("W_-1 50 bits", |b| {
        b.iter_custom(|iters| {
            let datas: Vec<f64> = (0..iters).map(|_| rng.gen_range(-1.0 / E..=0.0)).collect();
            let start = Instant::now();
            for &z in &datas {
                black_box(lambert_wm1(z));
            }
            start.elapsed()
        })
    });
    group.bench_function("W_-1 24 bits", |b| {
        b.iter_custom(|iters| {
            let datas: Vec<f64> = (0..iters).map(|_| rng.gen_range(-1.0 / E..=0.0)).collect();
            let start = Instant::now();
            for &z in &datas {
                black_box(sp_lambert_wm1(z));
            }
            start.elapsed()
        })
    });
    group.bench_function("W_-1 24 bits on f32", |b| {
        b.iter_custom(|iters| {
            let datas: Vec<f32> = (0..iters)
                .map(|_| rng.gen_range(-1.0 / E32..=0.0))
                .collect();
            let start = Instant::now();
            for &z in &datas {
                black_box(lambert_wm1f(z));
            }
            start.elapsed()
        })
    });
}

criterion_group!(benches, random_benches);
criterion_main!(benches);
