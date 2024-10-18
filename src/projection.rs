use faer::{Col, ColRef, MatRef};

#[inline]
pub fn vec_projection(vec: &ColRef<f32>, orthogonal: &MatRef<f32>) -> Col<f32> {
    orthogonal * vec
}

#[inline]
pub fn vec_projection_simd(vec: &ColRef<f32>, orthogonal: &MatRef<f32>) -> Col<f32> {
    Col::from_fn(orthogonal.ncols(), |i| unsafe {
        vector_dot_product(vec, &orthogonal.col(i))
    })
}

#[inline]
pub unsafe fn vector_dot_product(lhs: &ColRef<f32>, rhs: &ColRef<f32>) -> f32 {
    use std::arch::x86_64::*;

    let mut lhs_ptr = lhs.as_ptr();
    let mut rhs_ptr = rhs.as_ptr();
    let length = lhs.nrows();
    let rest = length & 0b111;
    let (mut vx, mut vy): (__m256, __m256);
    let mut accumulate = _mm256_setzero_ps();
    // let mut f32x8 = [0.0f32; 8];

    for _ in 0..(length / 16) {
        vx = _mm256_loadu_ps(lhs_ptr);
        vy = _mm256_loadu_ps(rhs_ptr);
        accumulate = _mm256_fmadd_ps(vx, vy, accumulate);
        lhs_ptr = lhs_ptr.add(8);
        rhs_ptr = rhs_ptr.add(8);

        vx = _mm256_loadu_ps(lhs_ptr);
        vy = _mm256_loadu_ps(rhs_ptr);
        accumulate = _mm256_fmadd_ps(vx, vy, accumulate);
        lhs_ptr = lhs_ptr.add(8);
        rhs_ptr = rhs_ptr.add(8);
    }
    for _ in 0..((length & 0b1111) / 8) {
        vx = _mm256_loadu_ps(lhs_ptr);
        vy = _mm256_loadu_ps(rhs_ptr);
        accumulate = _mm256_fmadd_ps(vx, vy, accumulate);
        lhs_ptr = lhs_ptr.add(8);
        rhs_ptr = rhs_ptr.add(8);
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

    let mut sum = reduce_f32_256(accumulate);

    for _ in 0..rest {
        sum += *lhs_ptr * *rhs_ptr;
        lhs_ptr = lhs_ptr.add(1);
        rhs_ptr = rhs_ptr.add(1);
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_projection() {
        use faer::Mat;

        let vec = Col::from_fn(3, |i| i as f32);
        let orthogonal = Mat::<f32>::identity(3, 3);
        let faer_res = vec_projection(&vec.as_ref(), &orthogonal.as_ref());
        let simd_res = vec_projection_simd(&vec.as_ref(), &orthogonal.as_ref());
        for i in 0..3 {
            // should be exact equal
            assert_eq!(faer_res[i], simd_res[i]);
        }
    }
}
