use criterion::{criterion_group, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};
use rs_bench::align::AlignedVec;
use rs_bench::binarize::{query_vector_binarize, vector_binarize_avx2};

pub fn binarize_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("binarize");
    for &size in [128, 256, 512, 1024].iter() {
        let x: Vec<u8> = (0..size).map(|_| rng.gen::<u8>()).collect();
        let mut y = AlignedVec::<u8>::new(size);
        y.fill_with(|| rng.gen::<u8>());
        group.bench_with_input(BenchmarkId::new("raw", size), &x, |b, input| {
            b.iter(|| query_vector_binarize(&input))
        });
        group.bench_with_input(BenchmarkId::new("avx2", size), &x, |b, input| {
            b.iter(|| unsafe { vector_binarize_avx2(&input) })
        });
        group.bench_with_input(BenchmarkId::new("aligned raw", size), &y, |b, input| {
            b.iter(|| query_vector_binarize(&input))
        });
        group.bench_with_input(BenchmarkId::new("aligned avx2", size), &y, |b, input| {
            b.iter(|| unsafe { vector_binarize_avx2(&input) })
        });
    }
    group.finish();
}

criterion_group!(binarize, binarize_benchmark);
