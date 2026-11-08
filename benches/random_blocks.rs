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

    let mut group = c.benchmark_group("random inputs within W0 blocks");

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
            format!("block {}/{}", i + 1, W0_CRITICAL_POINTS.len() - 1).as_str(),
            lambert_w0,
            w[0].next_up()..=w[1],
            &mut rng,
        );
    }

    drop(group);

    let mut group = c.benchmark_group("random inputs within SP_W0 blocks");

    const SP_W0_CRITICAL_POINTS: [f64; 25] = [
        -0.3678794411714423,
        -0.0675,
        -1e-9,
        1e-9,
        0.0675,
        0.15,
        2.008_217_811_584_472_656_3,
        30.539_142_109_510_895_244,
        371.669_843_713_757_760_70,
        4_705.918_954_265_969_037_7,
        64_640.797_355_310_089_675,
        965_649.030_871_163_226_41,
        1.559_333_422_803_816_549_0e7,
        2.702_564_027_724_190_315_7e8,
        4.995_018_739_704_194_635_5e9,
        9.791_115_441_672_696_027_7e10,
        2.025_975_385_630_209_968_3e12,
        4.407_744_425_147_793_893_9e13,
        1.004_838_215_057_150_448_5e15,
        2.393_255_260_235_983_768_7e16,
        5.939_799_659_746_575_434_6e17,
        1.532_693_858_990_176_659_2e19,
        4.103_565_939_888_539_413_5e20,
        2.172_370_661_049_060_431_7e141,
        f64::MAX,
    ];

    for (i, w) in SP_W0_CRITICAL_POINTS.windows(2).enumerate() {
        bench_on_vec_of_random_values_in_range(
            &mut group,
            format!("block {}/{}", i + 1, SP_W0_CRITICAL_POINTS.len() - 1).as_str(),
            sp_lambert_w0,
            w[0].next_up()..=w[1],
            &mut rng,
        );
    }

    drop(group);

    let mut group = c.benchmark_group("random inputs within W0F blocks");

    for (i, w) in SP_W0_CRITICAL_POINTS.windows(2).enumerate() {
        bench_on_vec_of_random_values_in_range(
            &mut group,
            format!("block {}/{}", i + 1, SP_W0_CRITICAL_POINTS.len() - 1).as_str(),
            lambert_w0f,
            (w[0] as f32).next_up()..=(w[1] as f32),
            &mut rng,
        );
    }

    drop(group);

    let mut group = c.benchmark_group("random inputs within WM1 blocks");

    const WM1_CRITICAL_POINTS: [f64; 12] = [
        -0.3678794411714423,
        -0.354_291_330_944_216_4,
        -0.188_726_882_822_894_340_49,
        -0.060_497_597_226_958_343_647,
        -0.017_105_334_740_676_008_194,
        -0.004_595_496_212_794_370_643_3,
        -0.001_200_161_067_219_772_417_3,
        -0.000_307_288_059_321_914_998_44,
        -0.000_077_447_159_838_062_184_354,
        -4.580_811_969_815_817_317_4e-17,
        -6.107_367_223_659_479_298_2e-79,
        0.0,
    ];

    for (i, w) in WM1_CRITICAL_POINTS.windows(2).enumerate() {
        bench_on_vec_of_random_values_in_range(
            &mut group,
            format!("block {}/{}", i + 1, WM1_CRITICAL_POINTS.len() - 1).as_str(),
            lambert_wm1,
            w[0].next_up()..=w[1],
            &mut rng,
        );
    }
}

criterion_group!(benches, random_benches_within_blocks);
criterion_main!(benches);
