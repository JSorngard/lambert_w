use criterion::{criterion_group, criterion_main, Criterion};
use lambert_w::w0;
use std::hint::black_box;

fn ln_vs_lambert_w_0(c: &mut Criterion) {
    let args = [
        -2.678794411714424e-01_f64,
        6.321205588285577e-01,
        9.632120558828557,
        9.999632120558828e+03,
        9.999999996321206e+08,
        9.999999999999633e+12,
        1.000000000000000e+18,
        1.000000000000000e+160,
    ];

    for z in args {
        let mut group = c.benchmark_group(format!("{z}"));
        group.bench_function(&format!("ln"), |b| b.iter(|| black_box(z.ln())));
        group.bench_function(&format!("W_0"), |b| b.iter(|| black_box(w0(z))));
    }
}

criterion_group!(benches, ln_vs_lambert_w_0);
criterion_main!(benches);
