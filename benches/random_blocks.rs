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

fn random_benches_within_blocks(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(0b1010101010101);

    let mut group = c.benchmark_group("random inputs within blocks");

    const W0_CRITICAL_POINTS: [f64; 25] = [
        -0.3678794411714423_f64,
        -0.0675,
        -1e-9,
        1e-9,
        0.0675,
        0.15,
        2.549_893_906_503_473_571_6,
        43.613_924_462_669_367_895,
        598.453_533_718_782_769_46,
        8_049.491_985_075_761_910_9,
        111_124.954_121_217_814_20,
        1.587_042_981_208_229_711_2e6,
        2.341_470_840_187_545_950_9e7,
        3.557_647_430_800_996_522_5e8,
        5.550_171_629_616_362_785_4e9,
        8.867_470_483_965_777_533_1e10,
        1.447_779_186_527_290_281_6e12,
        2.411_145_863_251_185_193_1e13,
        4.089_703_644_260_084_556_4e14,
        7.055_590_147_678_997_240_2e15,
        1.236_660_755_797_672_728_7e17,
        2.199_937_348_793_099_977_5e18,
        3.968_539_219_834_401_615_5e19,
        1.412_707_514_527_465_206_9e104,
        f64::MAX,
    ];

    for (i, w) in W0_CRITICAL_POINTS.windows(2).enumerate() {
        bench_on_vec_of_random_values_in_range(
            &mut group,
            format!("W0, block {i}/{}", W0_CRITICAL_POINTS.len() - 1).as_str(),
            lambert_w0,
            w[0].next_up()..=w[1],
            &mut rng,
        );
    }
}

criterion_group!(benches, random_benches_within_blocks);
criterion_main!(benches);
