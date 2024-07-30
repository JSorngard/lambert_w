use criterion::{criterion_group, criterion_main, Criterion};
use lambert_w::{lambert_w_0, lambert_w_m1, sp_lambert_w_0, sp_lambert_w_m1};
use std::hint::black_box;

fn ln_vs_lambert_w_0(c: &mut Criterion) {
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

    let small_args = [-0.2];

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
        group.bench_function(&format!("50 bits"), |b| {
            b.iter(|| black_box(lambert_w_m1(z)))
        });
        group.bench_function(&format!("24 bits"), |b| {
            b.iter(|| black_box(sp_lambert_w_m1(z)))
        });
    }
}

criterion_group!(benches, ln_vs_lambert_w_0);
criterion_main!(benches);
