use core::f32::consts::E as E32;
use core::f64::consts::E as E64;
use core::ops::RangeBounds;
use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use lambert_w::{lambert_w0, lambert_wm1};
use lambert_w::{lambert_w0f, lambert_wm1f, sp_lambert_w0, sp_lambert_wm1};
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    rngs::SmallRng,
    Rng, SeedableRng,
};
use std::time::Instant;

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
compile_error!(
    "at least one of the features 'std' and 'libm' must be active to benchmark anything"
);

/// Generates a vec of random values in the given range and benchmarks the given function
/// on those values.
///
/// It generates the random values before starting the time measurement in the benchmark,
/// and drops them after finishing, such that the generation and drop have no effect on the measured time.
fn bench_on_vec_of_random_values_in_range<'a, R, T, F, Prng>(
    group: &mut BenchmarkGroup<'a, WallTime>,
    id: &str,
    f: F,
    range: R,
    rng: &mut Prng,
) where
    R: Clone + RangeBounds<T> + SampleRange<T>,
    T: Copy + PartialOrd + SampleUniform,
    F: Fn(T) -> T,
    Prng: Rng,
{
    group.bench_function(id, |b| {
        b.iter_custom(|iters| {
            let datas: Vec<T> = (0..iters).map(|_| rng.gen_range(range.clone())).collect();
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
        "W_0 50 bits",
        lambert_w0,
        -1.0 / E64..f64::MAX,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 24 bits",
        sp_lambert_w0,
        -1.0 / E64..f64::MAX,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_0 24 bits on f32",
        lambert_w0f,
        -1.0 / E32..f32::MAX,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_-1 50 bits",
        lambert_wm1,
        -1.0 / E64..=0.0,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_-1 24 bits",
        sp_lambert_wm1,
        -1.0 / E64..=0.0,
        &mut rng,
    );

    bench_on_vec_of_random_values_in_range(
        &mut group,
        "W_-1 24 bits on f32",
        lambert_wm1f,
        -1.0 / E32..=0.0,
        &mut rng,
    );
}

criterion_group!(benches, random_benches);
criterion_main!(benches);
