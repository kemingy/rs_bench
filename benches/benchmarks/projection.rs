use criterion::{criterion_group, BenchmarkId, Criterion};
use faer::{Col, Mat};
use rand::{thread_rng, Rng};
use rs_bench::projection::{vec_projection, vec_projection_simd};

pub fn projection_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("projection");
    for &size in [64, 128, 256, 512, 960, 1024].iter() {
        let x = Col::from_fn(size, |_| rng.gen::<f32>());
        let orthogonal = Mat::<f32>::from_fn(size, size, |_, _| rng.gen())
            .qr()
            .compute_q();
        group.bench_with_input(
            BenchmarkId::new("faer", size),
            &(&x.as_ref(), &orthogonal.as_ref()),
            |b, input| b.iter(|| vec_projection(&input.0, &input.1)),
        );
        group.bench_with_input(
            BenchmarkId::new("simd", size),
            &(&x.as_ref(), &orthogonal.as_ref()),
            |b, input| b.iter(|| vec_projection_simd(&input.0, &input.1)),
        );
    }
    group.finish();
}

criterion_group!(project, projection_benchmark);
