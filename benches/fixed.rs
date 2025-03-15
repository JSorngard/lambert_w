use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lambert_w::{
    lambert_w, lambert_w0, lambert_w0f, lambert_wm1, lambert_wm1f, sp_lambert_w0, sp_lambert_wm1,
};

fn fixed_benches(c: &mut Criterion) {
    let big_args = [
        -2.678_794_411_714_424e-1_f64,
        9.632120558828557,
        9.999_632_120_558_828e3,
        1e18,
        1e160,
    ];

    let small_args = [
        -3.578_794_411_714_423e-1_f64,
        -3e-3,
        -3e-5,
        -1.000000000000004e-75,
        -1.000000000000008e-145,
    ];

    for z in big_args {
        let mut group = c.benchmark_group(format!("fixed W_0 at {z}"));
        group.bench_function("50 bits", |b| b.iter(|| black_box(lambert_w0(z))));
        group.bench_function("24 bits", |b| b.iter(|| black_box(sp_lambert_w0(z))));
        {
            let z32 = z as f32;
            if z32 < f32::INFINITY {
                group.bench_function("24 bits on f32", |b| b.iter(|| black_box(lambert_w0f(z32))));
            }
        }
        drop(group);
        let mut group = c.benchmark_group(format!("fixed Halley at {z}"));
        group.bench_function("branch 0", |b| b.iter(|| black_box(lambert_w(0, z, 0.0))));
    }
    for z in small_args {
        let mut group = c.benchmark_group(format!("fixed W_-1 at {z}"));
        group.bench_function("50 bits", |b| b.iter(|| black_box(lambert_wm1(z))));
        group.bench_function("24 bits", |b| b.iter(|| black_box(sp_lambert_wm1(z))));
        {
            let z32 = z as f32;
            if z32 < 0.0 {
                group.bench_function("24 bits on f32", |b| {
                    b.iter(|| black_box(lambert_wm1f(z32)))
                });
            }
        }
        drop(group);
        let mut group = c.benchmark_group(format!("fixed Halley at {z}"));
        group.bench_function("branch -1", |b| b.iter(|| black_box(lambert_w(-1, z, 0.0))));
    }
}

criterion_group!(benches, fixed_benches);
criterion_main!(benches);
