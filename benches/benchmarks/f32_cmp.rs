use criterion::{criterion_group, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};
use rs_bench::ord32::{Ord32, OrdF32};

fn find_minimal<T>(vec: &[T]) -> T
where
    T: Ord + Copy,
{
    let mut min = vec[0];
    for &x in vec.iter() {
        if x < min {
            min = x;
        }
    }
    min
}

pub fn f32_order_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("f32 cmp");
    for &size in [32, 1024, 65535].iter() {
        let x: Vec<f32> = (0..size).map(|_| rng.gen()).collect();
        let y = x.iter().map(|&x| x.into()).collect::<Vec<OrdF32>>();
        let z = x.iter().map(|&x| x.into()).collect::<Vec<Ord32>>();
        group.bench_with_input(BenchmarkId::new("ord f32", size), &y, |b, input| {
            b.iter(|| find_minimal(input))
        });
        group.bench_with_input(BenchmarkId::new("ord i32", size), &z, |b, input| {
            b.iter(|| find_minimal(&input))
        });
    }
    group.finish();
}

criterion_group!(f32_cmp, f32_order_benchmark);
