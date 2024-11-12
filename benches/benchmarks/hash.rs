use criterion::{criterion_group, BenchmarkId, Criterion};
use rs_bench::hash::{ahash, hashmap, nohash};

const EPOCH: usize = 1000;

pub fn hash_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash");

    for &size in [128, 1024, 4096, 16384].iter() {
        group.bench_with_input(BenchmarkId::new("std", size), &size, |b, input| {
            b.iter(|| hashmap(*input, EPOCH))
        });
        group.bench_with_input(BenchmarkId::new("nohash", size), &size, |b, input| {
            b.iter(|| nohash(*input, EPOCH))
        });
        group.bench_with_input(BenchmarkId::new("ahash", size), &size, |b, input| {
            b.iter(|| ahash(*input, EPOCH))
        });
    }

    group.finish();
}

criterion_group!(hash, hash_benchmark);
