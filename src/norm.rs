use faer::ColRef;

#[inline]
pub fn l2_squared_distance(lhs: &ColRef<f32>, rhs: &ColRef<f32>) -> f32 {
    (lhs - rhs).squared_norm_l2()
}

#[inline]
pub unsafe fn l2_squared_distance_simd(lhs: &ColRef<f32>, rhs: &ColRef<f32>) -> f32 {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    assert_eq!(lhs.nrows(), rhs.nrows());
    let mut lhs_ptr = lhs.as_ptr();
    let mut rhs_ptr = rhs.as_ptr();
    let block_16_num = lhs.nrows() >> 4;
    let rest_num = lhs.nrows() & 0b1111;
    let (mut diff, mut vx, mut vy): (__m256, __m256, __m256);
    let mut sum = _mm256_setzero_ps();

    for _ in 0..block_16_num {
        vx = _mm256_loadu_ps(lhs_ptr);
        vy = _mm256_loadu_ps(rhs_ptr);
        lhs_ptr = lhs_ptr.add(8);
        rhs_ptr = rhs_ptr.add(8);
        diff = _mm256_sub_ps(vx, vy);
        sum = _mm256_fmadd_ps(diff, diff, sum);

        vx = _mm256_loadu_ps(lhs_ptr);
        vy = _mm256_loadu_ps(rhs_ptr);
        lhs_ptr = lhs_ptr.add(8);
        rhs_ptr = rhs_ptr.add(8);
        diff = _mm256_sub_ps(vx, vy);
        sum = _mm256_fmadd_ps(diff, diff, sum);
    }

    for _ in 0..rest_num / 8 {
        vx = _mm256_loadu_ps(lhs_ptr);
        vy = _mm256_loadu_ps(rhs_ptr);
        lhs_ptr = lhs_ptr.add(8);
        rhs_ptr = rhs_ptr.add(8);
        diff = _mm256_sub_ps(vx, vy);
        sum = _mm256_fmadd_ps(diff, diff, sum);
    }

    #[inline(always)]
    unsafe fn reduce_f32_256(accumulate: __m256) -> f32 {
        // add [4..7] to [0..3]
        let mut combined = _mm256_add_ps(
            accumulate,
            _mm256_permute2f128_ps(accumulate, accumulate, 1),
        );
        // add [0..3] to [0..1]
        combined = _mm256_hadd_ps(combined, combined);
        // add [0..1] to [0]
        combined = _mm256_hadd_ps(combined, combined);
        _mm256_cvtss_f32(combined)
    }

    let mut res = reduce_f32_256(sum);
    for _ in 0..rest_num {
        let residual = *lhs_ptr - *rhs_ptr;
        res += residual * residual;
        lhs_ptr = lhs_ptr.add(1);
        rhs_ptr = rhs_ptr.add(1);
    }
    res
}

#[test]
fn test_l2_squared_distance() {
    use faer::Col;

    let x = Col::from_fn(3, |i| i as f32);
    let y = Col::from_fn(3, |i| (i + 1) as f32);
    assert_eq!(l2_squared_distance(&x.as_ref(), &y.as_ref()), 3.0);
    unsafe {
        assert_eq!(l2_squared_distance_simd(&x.as_ref(), &y.as_ref()), 3.0);
    }
}
