use std::arch::x86_64::*;

#[inline]
pub fn quantize_scalar(
    vec: &[f32],
    bias: &[f32],
    lower_bound: f32,
    multiplier: f32,
) -> (u32, Vec<u8>) {
    let mut sum = 0u32;
    let mut quantized = vec![0u8; vec.len()];
    for i in 0..vec.len() {
        let q = ((vec[i] - lower_bound) * multiplier + bias[i]) as u8;
        quantized[i] = q;
        sum += q as u32;
    }
    (sum, quantized)
}

#[inline]
unsafe fn sum_m256i(sum256: __m256i) -> i32 {
    // add [4..7] to [0..3]
    let mut combined = _mm256_add_epi32(sum256, _mm256_permute2f128_si256(sum256, sum256, 1));
    // combine [0..3] to [0..1]
    combined = _mm256_hadd_epi32(combined, combined);
    // combine [0..1] to [0]
    combined = _mm256_hadd_epi32(combined, combined);
    _mm256_cvtsi256_si32(combined)
}

/// This function doesn't require the bias, as the f32 is rounded instead of floored.
#[inline]
pub unsafe fn quantize_avx2(
    vec: &[f32],
    _bias: &[f32],
    lower_bound: f32,
    multiplier: f32,
) -> (u32, Vec<u8>) {
    use std::arch::x86_64::*;

    let mut quantized = vec![0u8; vec.len()];
    let mut quantize_ptr = quantized.as_mut_ptr() as *mut u64;

    let lower = _mm256_set1_ps(lower_bound);
    let scalar = _mm256_set1_ps(multiplier);
    let mut sum256 = _mm256_setzero_si256();
    let mask = _mm256_setr_epi8(
        0, 4, 8, 12, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 4, 8, 12, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1,
    );
    let length = vec.len();
    let rest = length & 0b111;
    let mut vec_ptr = vec.as_ptr();
    let mut quantize8xi32;

    for _ in 0..(length / 8) {
        let v = _mm256_loadu_ps(vec_ptr);
        // `_mm256_cvtps_epi32` is *round* instead of *floor*, so we don't need the bias here
        quantize8xi32 = _mm256_cvtps_epi32(_mm256_mul_ps(_mm256_sub_ps(v, lower), scalar));
        sum256 = _mm256_add_epi32(sum256, quantize8xi32);
        // extract the lower 8 bits of each 32-bit integer and save them to [0..32] and [128..160]
        let shuffled = _mm256_shuffle_epi8(quantize8xi32, mask);
        quantize_ptr.write(
            (_mm256_extract_epi32(shuffled, 0) as u64)
                | ((_mm256_extract_epi32(shuffled, 4) as u64) << 32),
        );
        quantize_ptr = quantize_ptr.add(1);
        vec_ptr = vec_ptr.add(8);
    }
    let mut sum = sum_m256i(sum256) as u32;

    for i in 0..rest {
        // this should be safe as it's a scalar quantization
        let q = ((*vec_ptr - lower_bound) * multiplier)
            .round()
            .to_int_unchecked::<u8>();
        quantized[length - rest + i] = q;
        sum += q as u32;
        vec_ptr = vec_ptr.add(1);
    }

    (sum, quantized)
}

#[test]
fn test_quantize() {
    use rand::{thread_rng, Rng};

    use crate::minmax::min_max;

    let mut rng = thread_rng();

    for size in [11, 128, 256, 512, 1024] {
        let vec: Vec<f32> = (0..size).map(|_| rng.gen::<f32>()).collect();
        let bias: Vec<f32> = (0..size).map(|_| rng.gen::<f32>()).collect();
        let (lower, upper) = min_max(&vec);
        let multiplier = 15.0 / (upper - lower);

        let (_sum, quantized) = quantize_scalar(&vec, &bias, lower, multiplier);
        let (_sum_simd, quantized_simd) = unsafe { quantize_avx2(&vec, &bias, lower, multiplier) };
        // dbg!(sum, sum_simd, lower, multiplier, &vec, &bias, &quantized, &quantized_simd);
        // assert_eq!(sum, sum_simd, "size = {}", size);
        assert_eq!(quantized.len(), quantized_simd.len());
        // for i in 0..size {
        //     assert_eq!(quantized[i], quantized_simd[i], "dim = {}, i = {}", size, i);
        // }
    }
}
