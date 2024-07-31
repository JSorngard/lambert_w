use core::f64::consts::E;
use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use lambert_w::{lambert_w_0, lambert_w_m1, sp_lambert_w_0, sp_lambert_w_m1};
use rand::{thread_rng, Rng};
use std::time::Instant;

fn bench(c: &mut Criterion) {
    let big_args = [
        -2.678794411714424e-01_f64,
        6.321205588285577e-01,
        9.632120558828557,
        9.999632120558828e+03,
        9.999999996321206e+08,
        9.999999999999633e+12,
        1.000000000000000e+18,
        1.000000000000000e+160,
    ];

    let small_args = [
        -3.578794411714423e-01_f64,
        -3.000000000000000e-02,
        -3.000000000000000e-03,
        -1.000000000000000e-04,
        -3.000000000000000e-05,
        -1.000000000000004e-75,
        -1.000000000000008e-145,
    ];

    {
        let mut group = c.benchmark_group("random inputs");
        let mut rng = thread_rng();
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

    for z in big_args {
        let mut group = c.benchmark_group(format!("W_0({z})"));
        group.bench_function(&format!("50 bits"), |b| {
            b.iter(|| black_box(lambert_w_0(z)))
        });
        group.bench_function(&format!("24 bits"), |b| {
            b.iter(|| black_box(sp_lambert_w_0(z)))
        });
    }
    for z in small_args {
        let mut group = c.benchmark_group(format!("W_-1({z})"));
        group.bench_function("50 bits", |b| b.iter(|| black_box(lambert_w_m1(z))));
        group.bench_function("24 bits", |b| b.iter(|| black_box(sp_lambert_w_m1(z))));
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
