const THETA_LOG_DIM: u32 = 4;

#[inline]
fn binary_dot_product(x: &[u64], y: &[u64]) -> u32 {
    let mut res = 0;
    for i in 0..x.len() {
        res += (x[i] & y[i]).count_ones();
    }
    res
}

pub fn asymmetric_binary_dot_product(x: &[u64], y: &[u64]) -> u32 {
    let mut res = 0;
    let length = x.len();
    assert_eq!(length, y.len() / THETA_LOG_DIM as usize);
    let mut ys = y;
    for i in 0..THETA_LOG_DIM as usize {
        res += binary_dot_product(x, ys) << i;
        ys = &ys[length..];
    }
    res
}

pub fn asymmetric_4_binary_ip(x: &[u64], y: &[&[u64]; 4]) -> u32 {
    let mut res = 0;
    res += binary_dot_product(x, y[0]);
    res += binary_dot_product(x, y[1]) << 1;
    res += binary_dot_product(x, y[2]) << 2;
    res += binary_dot_product(x, y[3]) << 3;
    res
}

#[inline]
fn binary_dot_product_block(x: &[u64], y: &[u64]) -> u32 {
    let mut res = 0;
    let block = x.len() / 8;
    let rest = x.len() & 0b111;
    for i in 0..block {
        for j in (i * 8)..((i + 1) * 8) {
            res += (x[j] & y[j]).count_ones();
        }
    }
    for i in 0..rest {
        res += (x[block * 8 + i] & y[block * 8 + i]).count_ones();
    }
    res
}

#[inline]
pub fn asymmetric_binary_dot_product_block(x: &[u64], y: &[u64]) -> u32 {
    let mut res = 0;
    let length = x.len();
    let mut ys = y;
    for i in 0..THETA_LOG_DIM as usize {
        res += binary_dot_product_block(x, ys) << i;
        ys = &ys[length..];
    }
    res
}

#[inline]
unsafe fn binary_ip_avx2(x: &[u64], y: &[u64]) -> u32 {
    use std::arch::x86_64::*;

    let mut sum = 0;
    let length = x.len() / 4;
    if length == 0 {
        for i in 0..x.len() {
            sum += (x[i] & y[i]).count_ones();
        }
        return sum;
    }
    let rest = x.len() & 0b11;
    for i in 0..rest {
        sum += (x[4 * length + i] & y[4 * length + i]).count_ones();
    }

    #[inline]
    unsafe fn mm256_popcnt_epi64(x: __m256i) -> __m256i {
        let lookup_table = _mm256_setr_epi8(
            0, 1, 1, 2, 1, 2, 2, 3, //
            1, 2, 2, 3, 2, 3, 3, 4, //
            0, 1, 1, 2, 1, 2, 2, 3, //
            1, 2, 2, 3, 2, 3, 3, 4, //
        );
        let mask = _mm256_set1_epi8(15);
        let zero = _mm256_setzero_si256();

        let mut low = _mm256_and_si256(x, mask);
        let mut high = _mm256_and_si256(_mm256_srli_epi64(x, 4), mask);
        low = _mm256_shuffle_epi8(lookup_table, low);
        high = _mm256_shuffle_epi8(lookup_table, high);
        _mm256_sad_epu8(_mm256_add_epi8(low, high), zero)
    }

    let mut sum256 = _mm256_setzero_si256();
    let mut x_ptr = x.as_ptr() as *const __m256i;
    let mut y_ptr = y.as_ptr() as *const __m256i;

    for _ in 0..length {
        let x256 = _mm256_loadu_si256(x_ptr);
        let y256 = _mm256_loadu_si256(y_ptr);
        let and = _mm256_and_si256(x256, y256);
        sum256 = _mm256_add_epi64(sum256, mm256_popcnt_epi64(and));
        x_ptr = x_ptr.add(1);
        y_ptr = y_ptr.add(1);
    }

    let xa = _mm_add_epi64(
        _mm256_castsi256_si128(sum256),
        _mm256_extracti128_si256(sum256, 1),
    );
    sum += _mm_cvtsi128_si64(_mm_add_epi64(xa, _mm_shuffle_epi32(xa, 78))) as u32;

    sum
}

#[inline]
pub fn asymmetric_binary_ip_avx2(x: &[u64], y: &[u64]) -> u32 {
    let mut res = 0;
    let length = x.len();
    let mut y_slice = y;
    for i in 0..THETA_LOG_DIM as usize {
        res += unsafe { binary_ip_avx2(x, &y_slice) << i };
        y_slice = &y_slice[length..];
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

#[inline]
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

#[inline]
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

#[inline]
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

#[inline]
fn binary_ip_popcnt(x: &[u64], y: &[u64], z: &mut [u64]) -> u32 {
    for i in 0..x.len() {
        z[i] = x[i] & y[i];
    }
    popcnt::count_ones(bytemuck::cast_slice(z)) as u32
}

pub fn asymmetric_binary_ip_popcnt(x: &[u64], y: &[u64]) -> u32 {
    let mut res = 0;
    let length = x.len();
    let mut ys = y;
    let mut z = vec![0u64; length];
    for i in 0..THETA_LOG_DIM as usize {
        res += binary_ip_popcnt(x, ys, &mut z) << i;
        ys = &ys[length..];
    }
    res
}

#[test]
fn test_binary_dot_product() {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    for length in [2, 4, 15, 16, 64] {
        let x = (0..length).map(|_| rng.gen::<u64>()).collect::<Vec<u64>>();
        let y = (0..length * 4)
            .map(|_| rng.gen::<u64>())
            .collect::<Vec<u64>>();

        let truth = asymmetric_binary_dot_product(&x, &y);
        assert_eq!(
            truth,
            asymmetric_binary_dot_product_block(&x, &y),
            "length = {}",
            length
        );
        assert_eq!(
            truth,
            asymmetric_binary_ip_one(&x, &y),
            "length = {}",
            length
        );
        assert_eq!(
            truth,
            asymmetric_binary_ip_avx2(&x, &y),
            "length = {}",
            length
        );
        // assert_eq!(
        //     truth,
        //     asymmetric_binary_ip_simd(&x, &y),
        //     "length = {}",
        //     length
        // );
    }
}
