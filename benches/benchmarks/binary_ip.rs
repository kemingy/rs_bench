#![allow(dead_code)]
use criterion::{criterion_group, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};
use rs_bench::binary_ip::{asymmetric_binary_dot_product, asymmetric_binary_ip_avx2};

pub fn binary_ip_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("binary ip");
    for &size in [2, 4, 15, 16].iter() {
        let x = (0..size).map(|_| rng.gen::<u64>()).collect::<Vec<u64>>();
        let y = (0..size * 4)
            .map(|_| rng.gen::<u64>())
            .collect::<Vec<u64>>();
        group.bench_with_input(
            BenchmarkId::from_parameter(size * 64),
            &(&x, &y),
            |b, input| b.iter(|| asymmetric_binary_dot_product(&input.0, &input.1)),
        );
        group.bench_with_input(
            BenchmarkId::new("avx2", size * 64),
            &(&x, &y),
            |b, input| b.iter(|| asymmetric_binary_ip_avx2(&input.0, &input.1)),
        );
    }
    group.finish();
}

criterion_group!(binary_ip, binary_ip_benchmark);
