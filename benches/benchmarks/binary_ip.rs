#[allow(dead_code)]
use criterion::{criterion_group, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};

const THETA_LOG_DIM: u32 = 4;

#[inline]
fn binary_dot_product(x: &[u64], y: &[u64]) -> u32 {
    // x.iter().zip(y.iter()).map(|(a, b)| (a & b).count_ones()).sum()
    // assert!(x.len() == y.len());
    let mut res = 0;
    for i in 0..x.len() {
        res += (x[i] & y[i]).count_ones();
    }
    res
}

// binary_dot_product(x, y) + binary_dot_product(x, &y[length..]) << 1 + binary_dot_product(x, &y[length*2..]) << 2 + binary_dot_product(x, &y[length*3..]) << 3
pub fn asymmetric_binary_dot_product(x: &[u64], y: &[u64]) -> u32 {
    let mut res = 0;
    let length = x.len();
    // assert_eq!(length, y.len() / THETA_LOG_DIM as usize);
    let mut ys = y;
    for i in 0..THETA_LOG_DIM as usize {
        res += binary_dot_product(x, ys) << i;
        ys = &ys[length..];
    }
    res
}

#[inline]
unsafe fn binary_dot_product_popcount(x: &[u64], y: &[u64]) -> i32 {
    let mut res = 0;
    for i in 0..x.len() {
        res += std::arch::x86_64::_popcnt64((x[i] & y[i]) as i64);
    }
    res
}

pub fn asymmetric_binary_dot_product_popcount(x: &[u64], y: &[u64]) -> u32 {
    let mut res = 0;
    let length = x.len();
    assert_eq!(length, y.len() / THETA_LOG_DIM as usize);
    unsafe {
        for i in 0..THETA_LOG_DIM as usize {
            res += binary_dot_product_popcount(x, &y[i * length..]) << i;
        }
    }
    res as u32
}

pub fn asymmetric_binary_ip_one(x: &[u64], y: &[u64]) -> u32 {
    let mut res = 0;
    let length = x.len();
    let mut yslice = y;
    for i in 0..THETA_LOG_DIM as usize {
        res += x
            .iter()
            .zip(yslice.iter())
            .map(|(a, b)| (a & b).count_ones())
            .sum::<u32>()
            << i;
        yslice = &yslice[length..];
    }
    res
}

#[inline]
pub unsafe fn binary_ip_simd(x: &[u64], y: &[u64]) -> u32 {
    use std::arch::x86_64::*;

    let mut res = 0;
    let length = x.len() / 4;
    let x_ptr = x.as_ptr() as *const __m256i;
    let y_ptr = y.as_ptr() as *const __m256i;
    for i in 0..length {
        let x_vec = _mm256_loadu_si256(x_ptr.add(i));
        let y_vec = _mm256_loadu_si256(y_ptr.add(i));
        let and = _mm256_and_si256(x_vec, y_vec);
        res += _popcnt64(_mm256_extract_epi64(and, 0))
            + _popcnt64(_mm256_extract_epi64(and, 1))
            + _popcnt64(_mm256_extract_epi64(and, 2))
            + _popcnt64(_mm256_extract_epi64(and, 3));
    }
    res as u32
}

pub fn asymmetric_binary_ip_simd(x: &[u64], y: &[u64]) -> u32 {
    let mut res = 0;
    let length = x.len();
    for i in 0..THETA_LOG_DIM as usize {
        unsafe {
            res += binary_ip_simd(x, &y[i * length..(i + 1) * length]) << i;
        }
    }
    res
}

#[test]
fn test_binary_dot_product() {
    let x = vec![
        0b1010, 0b0101, 0b1010, 0b0101, 0b1010, 0b0101, 0b1010, 0b0101,
    ];
    let y = vec![
        0b1100, 0b0011, 0b1100, 0b0011, 0b1100, 0b0011, 0b1100, 0b0011,
    ];
    assert_eq!(binary_dot_product(&x, &y), 8);
    assert_eq!(unsafe { binary_ip_simd(&x, &y) }, 8);
}

pub fn binary_ip_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    let mut group = c.benchmark_group("binary ip");
    for &size in [4usize, 16].iter() {
        let x = (0..size).map(|_| rng.gen::<u64>()).collect::<Vec<u64>>();
        let y = (0..size * 4)
            .map(|_| rng.gen::<u64>())
            .collect::<Vec<u64>>();
        group.bench_with_input(
            BenchmarkId::from_parameter(size * 64),
            &(x, y),
            |b, input| b.iter(|| asymmetric_binary_dot_product(&input.0, &input.1)),
        );
    }
    group.finish();
}

criterion_group!(binary_ip, binary_ip_benchmark);
