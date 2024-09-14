use core::f32;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(32))]
pub struct Aligned32<T>(pub T);

#[inline]
pub fn min_max(vec: &[f32]) -> (f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    for &x in vec.iter() {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    (min, max)
}

#[inline]
pub unsafe fn min_max_avx(vec: &[f32]) -> (f32, f32) {
    use std::arch::x86_64::*;

    let mut min_32x8 = _mm256_set1_ps(f32::MAX);
    let mut max_32x8 = _mm256_set1_ps(f32::MIN);
    let mut ptr = vec.as_ptr();
    let mut f32x8 = Aligned32([0.0f32; 8]);
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    let length = vec.len();
    let rest = length & 0b111;

    for _ in 0..(length / 8) {
        let v = _mm256_loadu_ps(ptr);
        ptr = ptr.add(8);
        min_32x8 = _mm256_min_ps(min_32x8, v);
        max_32x8 = _mm256_max_ps(max_32x8, v);
    }
    _mm256_store_ps(f32x8.0.as_mut_ptr(), min_32x8);
    for &x in f32x8.0.iter() {
        if x < min {
            min = x;
        }
    }
    _mm256_store_ps(f32x8.0.as_mut_ptr(), max_32x8);
    for &x in f32x8.0.iter() {
        if x > max {
            max = x;
        }
    }

    for _ in 0..rest {
        if *ptr < min {
            min = *ptr;
        }
        if *ptr > max {
            max = *ptr;
        }
        ptr = ptr.add(1);
    }

    (min, max)
}

#[test]
fn test_min_max() {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();

    for size in [64, 128, 164, 256, 512, 1024].iter() {
        let vec: Vec<f32> = (0..*size).map(|_| rng.gen::<f32>()).collect();
        assert_eq!(min_max(&vec), unsafe { min_max_avx(&vec) });
    }
}
