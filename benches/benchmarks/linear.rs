use criterion::{criterion_group, Criterion};
use rs_bench::linear::{
    DefaultMapStruct, LinearMapStruct, LinearMapTrait, MicroMapStruct, SmallMapStruct,
};

const EPOCH: usize = 1000;

fn test<const N: usize, L: LinearMapTrait>() {
    let mut map = L::new();
    for i in 0..N {
        map.insert(i as u32, i.to_string());
    }

    for _ in 0..EPOCH {
        for i in 0..N {
            map.get(i as u32);
        }
    }
}

pub fn linear_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("linear");

    group.bench_function("2 default", |b| b.iter(|| test::<2, DefaultMapStruct<2>>()));
    group.bench_function("2 micro", |b| b.iter(|| test::<2, MicroMapStruct<2>>()));
    group.bench_function("2 small", |b| b.iter(|| test::<2, SmallMapStruct<2>>()));
    group.bench_function("2 linear", |b| b.iter(|| test::<2, LinearMapStruct<2>>()));

    group.bench_function("4 default", |b| b.iter(|| test::<4, DefaultMapStruct<4>>()));
    group.bench_function("4 micro", |b| b.iter(|| test::<4, MicroMapStruct<4>>()));
    group.bench_function("4 small", |b| b.iter(|| test::<4, SmallMapStruct<4>>()));
    group.bench_function("4 linear", |b| b.iter(|| test::<4, LinearMapStruct<4>>()));

    group.bench_function("8 default", |b| b.iter(|| test::<8, DefaultMapStruct<8>>()));
    group.bench_function("8 micro", |b| b.iter(|| test::<8, MicroMapStruct<8>>()));
    group.bench_function("8 small", |b| b.iter(|| test::<8, SmallMapStruct<8>>()));
    group.bench_function("8 linear", |b| b.iter(|| test::<8, LinearMapStruct<8>>()));

    group.bench_function("16 default", |b| {
        b.iter(|| test::<16, DefaultMapStruct<16>>())
    });
    group.bench_function("16 micro", |b| b.iter(|| test::<16, MicroMapStruct<16>>()));
    group.bench_function("16 small", |b| b.iter(|| test::<16, SmallMapStruct<16>>()));
    group.bench_function("16 linear", |b| {
        b.iter(|| test::<16, LinearMapStruct<16>>())
    });

    group.bench_function("32 default", |b| {
        b.iter(|| test::<32, DefaultMapStruct<32>>())
    });
    group.bench_function("32 micro", |b| b.iter(|| test::<32, MicroMapStruct<32>>()));
    group.bench_function("32 small", |b| b.iter(|| test::<32, SmallMapStruct<32>>()));
    group.bench_function("32 linear", |b| {
        b.iter(|| test::<32, LinearMapStruct<32>>())
    });

    group.finish();
}

criterion_group!(linear, linear_benchmark);
