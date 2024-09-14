use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Ord32(i32);

impl Ord32 {
    #[inline]
    pub const fn from_f32(x: f32) -> Self {
        let bits = x.to_bits() as i32;
        let mask = ((bits >> 31) as u32) >> 1;
        let res = bits ^ (mask as i32);
        Self(res)
    }
}

impl From<f32> for Ord32 {
    #[inline]
    fn from(x: f32) -> Self {
        Self::from_f32(x)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OrdF32(f32);

impl From<f32> for OrdF32 {
    #[inline]
    fn from(x: f32) -> Self {
        Self(x)
    }
}

impl Eq for OrdF32 {}

impl Ord for OrdF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}
