use num_traits::Num;

use std::alloc::{alloc, dealloc, Layout};

// https://rust-lang.github.io/hashbrown/src/crossbeam_utils/cache_padded.rs.html#128-130
pub const CACHELINE_ALIGN: usize = {
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64",
    ))]
    {
        128
    }
    #[cfg(any(
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "riscv64",
    ))]
    {
        32
    }
    #[cfg(target_arch = "s390x")]
    {
        256
    }
    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64",
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "riscv64",
        target_arch = "s390x",
    )))]
    {
        64
    }
};

#[inline(always)]
pub fn align_for<T: 'static>() -> usize {
    if core::mem::size_of::<T>() % 8 == 0 {
        Ord::max(
            core::mem::size_of::<T>(),
            Ord::max(core::mem::align_of::<T>(), CACHELINE_ALIGN),
        )
    } else {
        core::mem::align_of::<T>()
    }
}

#[repr(C)]
pub struct AlignedVec<T: 'static> {
    ptr: *mut T,
    len: usize,
}

impl<T: 'static> AlignedVec<T>
where
    T: Num,
{
    pub fn new(len: usize) -> Self {
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), align_for::<T>())
            .expect("failed to create layout");
        unsafe {
            let ptr = alloc(layout) as *mut T;
            if ptr.is_null() {
                panic!("failed to allocate memory");
            }
            let mut vec = Self { ptr, len };
            // memset 0
            vec.iter_mut().for_each(|m| *m = T::zero());
            vec
        }
    }
}

impl<T: 'static> Drop for AlignedVec<T> {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.len * std::mem::size_of::<T>(), align_for::<T>())
            .expect("failed to create layout");
        unsafe {
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}

impl<T> std::ops::Deref for AlignedVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> std::ops::DerefMut for AlignedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
