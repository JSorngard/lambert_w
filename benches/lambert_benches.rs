use criterion::{criterion_group, criterion_main, Criterion};
use lambert_w::sp_lambert_w_m1;
use std::hint::black_box;

fn ln_vs_lambert_w_0(c: &mut Criterion) {
    let args = [
        -3.578794411714423e-01_f64,
        -3.000000000000000e-02,
        -3.000000000000000e-03,
        -1.000000000000000e-04,
        -3.000000000000000e-05,
        -1.000000000000004e-75,
        -1.000000000000008e-145,
    ];

    for z in args {
        //let mut group = c.benchmark_group(format!("{z}"));
        c.bench_function(&format!("W_m1 {z}"), |b| {
            b.iter(|| black_box(sp_lambert_w_m1(z)))
        });
    }
}

criterion_group!(benches, ln_vs_lambert_w_0);
criterion_main!(benches);
