use criterion::{criterion_group, BenchmarkId, Criterion};
use num_traits::Float;
use rand::{thread_rng, Rng};
use rs_binary_ip_simd::float32::F32;

pub fn quantize_15(lut: &[F32]) -> (F32, F32, Vec<u8>) {
    let min = lut.iter().copied().fold(F32::infinity(), std::cmp::min);
    let max = lut.iter().copied().fold(F32::neg_infinity(), std::cmp::max);
    let k = std::cmp::max(max - min, F32(0.0)) / F32(15.0);
    let b = min;
    (k, b, lut.iter().map(|&y| ((y - b) / k).0 as u8).collect())
}

pub fn quantize_raw(vec: &[f32]) -> (f32, f32, Vec<u8>) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    for &y in vec.iter() {
        min = min.min(y);
        max = max.max(y);
    }
    let k = 15.0 / (max - min + 1e-6);
    (k, min, vec.iter().map(|&x| ((x - min) / k) as u8).collect())
}

pub fn quantize_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("quantize");
    for &dim in [128usize, 256, 512, 1024].iter() {
        let raw = (0..dim).map(|_| rng.gen::<f32>()).collect::<Vec<f32>>();
        let scalar = (0..dim)
            .map(|_| F32::from(rng.gen::<f32>()))
            .collect::<Vec<F32>>();
        group.bench_with_input(BenchmarkId::new("raw", &dim), &raw, |b, input| {
            b.iter(|| quantize_raw(&input))
        });
        group.bench_with_input(BenchmarkId::new("scalar", &dim), &scalar, |b, input| {
            b.iter(|| quantize_15(&input))
        });
    }
    group.finish();
}

criterion_group!(quantize, quantize_benchmark);
