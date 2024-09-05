use criterion::{criterion_group, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};
use rs_bench::minmax::min_max;
use rs_bench::quantize::{quantize_avx2, quantize_scalar, quantize_scalar_round};

pub fn quantize_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("quantize");
    for &dim in [100, 128, 256, 512, 1024].iter() {
        let vec: Vec<f32> = (0..dim).map(|_| rng.gen::<f32>()).collect();
        let bias: Vec<f32> = (0..dim).map(|_| rng.gen::<f32>()).collect();
        let (lower, upper) = min_max(&vec);
        let multiplier = 15.0 / (upper - lower);
        group.bench_with_input(
            BenchmarkId::new("floor", &dim),
            &(&vec, &bias, lower, multiplier),
            |b, input| b.iter(|| quantize_scalar(&input.0, &input.1, input.2, input.3)),
        );
        group.bench_with_input(
            BenchmarkId::new("round", &dim),
            &(&vec, &bias, lower, multiplier),
            |b, input| b.iter(|| quantize_scalar_round(&input.0, &input.1, input.2, input.3)),
        );
        group.bench_with_input(
            BenchmarkId::new("simd", &dim),
            &(&vec, &bias, lower, multiplier),
            |b, input| b.iter(|| unsafe { quantize_avx2(&input.0, &input.1, input.2, input.3) }),
        );
    }
    group.finish();
}

criterion_group!(quantize, quantize_benchmark);
