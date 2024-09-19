use criterion::{criterion_group, BenchmarkId, Criterion};
use faer::Col;
use rand::{thread_rng, Rng};
use rs_bench::norm::{l2_squared_distance, l2_squared_distance_simd};

pub fn l2_square_distance_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("norm");
    for &size in [64, 128, 256, 512, 960, 1024].iter() {
        let x = Col::from_fn(size, |_| rng.gen::<f32>());
        let y = Col::from_fn(size, |_| rng.gen::<f32>());
        group.bench_with_input(
            BenchmarkId::new("faer", size),
            &(&x.as_ref(), &y.as_ref()),
            |b, input| b.iter(|| l2_squared_distance(&input.0, &input.1)),
        );
        group.bench_with_input(
            BenchmarkId::new("simd", size),
            &(&x.as_ref(), &y.as_ref()),
            |b, input| b.iter(|| unsafe { l2_squared_distance_simd(&input.0, &input.1) }),
        );
    }
    group.finish();
}

criterion_group!(norm, l2_square_distance_benchmark);
