use criterion::{criterion_group, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};
use rs_bench::minmax::{min_max, min_max_avx};

pub fn min_max_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("min_max");
    for &size in [128, 256, 512, 1024].iter() {
        let x: Vec<f32> = (0..size).map(|_| rng.gen::<f32>()).collect();
        group.bench_with_input(BenchmarkId::new("raw", size), &x, |b, input| {
            b.iter(|| min_max(&input))
        });
        group.bench_with_input(BenchmarkId::new("avx", size), &x, |b, input| {
            b.iter(|| unsafe { min_max_avx(&input) })
        });
    }
    group.finish();
}

criterion_group!(minmax, min_max_benchmark);
