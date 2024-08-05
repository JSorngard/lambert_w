use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use lambert_w::{lambert_w0, lambert_wm1, sp_lambert_w0, sp_lambert_wm1};

fn fixed_benches(c: &mut Criterion) {
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

    for z in big_args {
        let mut group = c.benchmark_group(format!("W_0({z})"));
        group.bench_function(&format!("50 bits"), |b| b.iter(|| black_box(lambert_w0(z))));
        group.bench_function(&format!("24 bits"), |b| {
            b.iter(|| black_box(sp_lambert_w0(z)))
        });
    }
    for z in small_args {
        let mut group = c.benchmark_group(format!("W_-1({z})"));
        group.bench_function("50 bits", |b| b.iter(|| black_box(lambert_wm1(z))));
        group.bench_function("24 bits", |b| b.iter(|| black_box(sp_lambert_wm1(z))));
    }
}

criterion_group!(benches, fixed_benches);
criterion_main!(benches);
