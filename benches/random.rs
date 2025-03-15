use core::ops::RangeBounds;
use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use lambert_w::{
    lambert_w, lambert_w0, lambert_w0f, lambert_wm1, lambert_wm1f, sp_lambert_w0, sp_lambert_wm1,
    NEG_INV_E,
};
use rand::{
    distr::uniform::{SampleRange, SampleUniform},
    rngs::SmallRng,
    Rng, SeedableRng,
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
    Prng: Rng,
{
    group.bench_function(id, |b| {
        b.iter_custom(|iters| {
            let datas: Vec<T> = (0..iters)
                .map(|_| rng.random_range(range.clone()))
                .collect();
            let start = Instant::now();
            for &z in &datas {
                black_box(f(z));
            }
            let duration = start.elapsed();
            drop(datas);
            duration
        })
    });
}

fn random_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("random inputs");
    let mut rng = SmallRng::seed_from_u64(0b1010101010101);

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 Halley iteration",
        |z| lambert_w(0, z, 0.0),
        NEG_INV_E..=f64::MAX,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 50 bits",
        lambert_w0,
        NEG_INV_E..f64::MAX,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 24 bits",
        sp_lambert_w0,
        NEG_INV_E..f64::MAX,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 24 bits on f32",
        lambert_w0f,
        NEG_INV_E as f32..f32::MAX,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_-1 Halley iteration",
        |z| lambert_w(-1, z, 0.0),
        NEG_INV_E..=0.0,
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
}

criterion_group!(benches, random_benches);
criterion_main!(benches);
