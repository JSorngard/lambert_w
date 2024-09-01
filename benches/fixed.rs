use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use lambert_w::{lambert_w0, lambert_wm1, sp_lambert_w0, sp_lambert_wm1};

fn fixed_benches(c: &mut Criterion) {
    let big_args = [
        -2.678_794_411_714_424e-1_f64,
        6.321_205_588_285_577e-1,
        9.632120558828557,
        9.999_632_120_558_828e3,
        9.999_999_996_321_206e8,
        9.999_999_999_999_633e12,
        1e18,
        1e160,
    ];

    let small_args = [
        -3.578_794_411_714_423e-1_f64,
        -3e-2,
        -3e-3,
        -1e-4,
        -3e-5,
        -1.000000000000004e-75,
        -1.000000000000008e-145,
    ];

    for z in big_args {
        let mut group = c.benchmark_group(format!("W_0({z})"));
        group.bench_function(&"50 bits".to_string(), |b| b.iter(|| black_box(lambert_w0(z))));
        group.bench_function(&"24 bits".to_string(), |b| {
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
