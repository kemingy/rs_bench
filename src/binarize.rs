#![allow(dead_code)]

const THETA_LOG_DIM: u32 = 4;

pub fn query_vector_binarize(vec: &[u8]) -> Vec<u64> {
    let length = vec.len();
    let mut binary = vec![0u64; length * THETA_LOG_DIM as usize / 64];
    for j in 0..THETA_LOG_DIM as usize {
        for i in 0..length {
            binary[(i + j * length) / 64] |= (((vec[i] >> j) & 1) as u64) << (63 - i % 64);
        }
    }
    binary
}

pub unsafe fn vector_binarize_avx2(vec: &[u8]) -> Vec<u64> {
    use std::arch::x86_64::*;

    let length = vec.len();
    let mut ptr = vec.as_ptr() as *const __m256i;
    let mut binary = vec![0u64; length * THETA_LOG_DIM as usize / 64];

    for i in (0..length).step_by(32) {
        let mut v = _mm256_load_si256(ptr);
        ptr = ptr.add(1);
        v = _mm256_slli_epi32(v, 4);
        for j in 0..THETA_LOG_DIM as usize {
            let mask = (_mm256_movemask_epi8(v) as u32).reverse_bits() as u64;
            // let shift = ((i >> 5) & 1) << 5;
            let shift = if (i / 32) % 2 == 0 { 32 } else { 0 };
            binary[(3 - j) * (length >> 6) + (i >> 6)] |= mask << shift;
            v = _mm256_slli_epi32(v, 1);
        }
    }

    binary
}

#[test]
fn test_binarize() {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    for size in [960usize].into_iter() {
        let vec: Vec<u8> = (0..size).map(|_| rng.gen::<u8>()).collect();
        assert_eq!(query_vector_binarize(&vec), unsafe {
            vector_binarize_avx2(&vec)
        });
    }
    // let vec: Vec<u8> = (0..128).map(|_| rng.gen::<u8>()).collect();
    // assert_eq!(query_vector_binarize(&vec), unsafe {
    //     vector_binarize_avx2(&vec)
    // });
}
