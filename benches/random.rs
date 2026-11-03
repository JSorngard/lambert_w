// Copyright 2024-2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Benchmarks for random arguments generated at runtime (from a fixed seed).

// The call to `criterion_group!` generates items that we can not document,
// and we can not mark the macro call itself.
#![allow(missing_docs)]

use core::{hint::black_box, ops::RangeBounds};
use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use lambert_w::{
    lambert_w, lambert_w0, lambert_w0f, lambert_wf, lambert_wm1, lambert_wm1f, sp_lambert_w0,
    sp_lambert_wm1, NEG_INV_E,
};
use rand::{
    distr::uniform::{SampleRange, SampleUniform},
    rngs::SmallRng,
    RngExt, SeedableRng,
};
use std::time::Instant;

/// Generates a vec of random values in the given range and benchmarks the given function
/// on those values.
///
/// It generates the random values before starting the time measurement in the benchmark,
/// and drops them after finishing, such that the generation and drop have no effect on the measured time.
fn bench_on_vec_of_random_values_in_range<'a, R, T, F, U, Prng>(
    group: &mut BenchmarkGroup<'a, WallTime>,
    id: &str,
    f: F,
    range: R,
    rng: &mut Prng,
) where
    R: Clone + RangeBounds<T> + SampleRange<T>,
    T: Copy + PartialOrd + SampleUniform,
    F: Fn(T) -> U,
    Prng: RngExt,
{
    group.bench_function(id, |b| {
        b.iter_custom(|iters| {
            let data: Vec<T> = (0..iters)
                .map(|_| rng.random_range(range.clone()))
                .collect();
            let start = Instant::now();
            for &z in &data {
                black_box(f(z));
            }
            let duration = start.elapsed();
            drop(data);
            duration
        })
    });
}

fn random_benches(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(0b1010101010101);

    let mut reference_group = c.benchmark_group("random inputs (Reference)");

    bench_on_vec_of_random_values_in_range(
        &mut reference_group,
        "ln",
        |x| x.ln(),
        0.1..=f64::from(u32::MAX),
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut reference_group,
        "sqrt",
        |x| x.sqrt(),
        0.1..=f64::from(u32::MAX),
        &mut rng,
    );

    drop(reference_group);

    let mut halley_group = c.benchmark_group("random inputs (Halley's method)");

    let tol = f64::EPSILON;
    let tolf = f32::EPSILON;

    bench_on_vec_of_random_values_in_range(
        &mut halley_group,
        "W_0",
        |z| lambert_w(0, z, 0.0, tol),
        NEG_INV_E..=f64::from(u32::MAX),
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut halley_group,
        "W_0 on 32-bit",
        |z| lambert_wf(0, z, 0.0, tolf),
        NEG_INV_E as f32..=f32::from(u16::MAX),
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut halley_group,
        "W_-1",
        |z| lambert_w(-1, z, 0.0, tol),
        NEG_INV_E..=0.0,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut halley_group,
        "W_-1 on 32-bit",
        |z| lambert_wf(-1, z, 0.0, tolf),
        NEG_INV_E as f32..=0.0,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut halley_group,
        "W_1000",
        |z| lambert_w(1_000, z, 0.0, tol),
        NEG_INV_E..=f64::from(u32::MAX),
        &mut rng,
    );

    drop(halley_group);

    let mut group = c.benchmark_group("random inputs (Fukushima's method)");

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 50 bits",
        lambert_w0,
        NEG_INV_E..f64::from(u32::MAX),
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 24 bits",
        sp_lambert_w0,
        NEG_INV_E..f64::from(u32::MAX),
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 24 bits on f32",
        lambert_w0f,
        NEG_INV_E as f32..f32::from(u16::MAX),
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_-1 50 bits",
        lambert_wm1,
        NEG_INV_E..=0.0,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_-1 24 bits",
        sp_lambert_wm1,
        NEG_INV_E..=0.0,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_-1 24 bits on f32",
        lambert_wm1f,
        NEG_INV_E as f32..=0.0,
        &mut rng,
    );

    drop(group);

    let mut group = c.benchmark_group("random inputs within blocks (Fukushima's method)");

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 50 bits around zero",
        lambert_w0,
        -0.0675..=0.0675,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 50 bits early block",
        lambert_w0,
        2.549_893_906_503_473_571_6_f64.next_up()..=43.613_924_462_669_367_895,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 50 bits middle block",
        lambert_w0,
        5.550_171_629_616_362_785_4e9_f64.next_up()..=8.867_470_483_965_777_533_1e10,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 50 bits late block",
        lambert_w0,
        1.412_707_514_527_465_206_9e104_f64.next_up()..f64::MAX,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 24 bits early block",
        sp_lambert_w0,
        2.008_217_811_584_472_656_3_f64.next_up()..30.539_142_109_510_895_244,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 24 bits middle block",
        sp_lambert_w0,
        4.995_018_739_704_194_635_5e9_f64.next_up()..9.791_115_441_672_696_027_7e10,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 24 bits late block",
        sp_lambert_w0,
        4.103_565_939_888_539_413_5e20_f64.next_up()..f64::MAX,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 24 bits on f32 early block",
        lambert_w0f,
        2.008_217_8_f32.next_up()..30.539_143,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 24 bits on f32 middle block",
        lambert_w0f,
        4.995_019e9_f32.next_up()..9.791_115e10,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 24 bits on f32 late block",
        lambert_w0f,
        4.103_566e20_f32.next_up()..f32::MAX,
        &mut rng,
    );
}

criterion_group!(benches, random_benches);
criterion_main!(benches);
