use core::alloc::Layout;

#[allow(dead_code)]
enum AllocHint {
    Unshareable,
    ShareableZeroed,
    UnshareableZeroed,
}

impl Into<u32> for AllocHint {
    fn into(self) -> u32 {
        //the spec document specified these 2 other hints but
        // when using them TEE_Malloc panics...
        use crate::wrapper::raw::TEE_MALLOC_FILL_ZERO;
        const TEE_MALLOC_NO_SHARE: u32 = 0b10;
        const TEE_MALLOC_NO_FILL: u32 = 0b01;

        match self {
            AllocHint::ShareableZeroed => TEE_MALLOC_FILL_ZERO,
            AllocHint::Unshareable => TEE_MALLOC_NO_FILL | TEE_MALLOC_NO_SHARE,
            AllocHint::UnshareableZeroed => TEE_MALLOC_FILL_ZERO | TEE_MALLOC_NO_SHARE,
        };

        //so the actual only available hint is the following...
        TEE_MALLOC_FILL_ZERO
    }
}

pub struct TEEAllocator;

impl TEEAllocator {
    fn alloc_impl(size: usize, hint: AllocHint) -> *mut u8 {
        unsafe { crate::wrapper::raw::TEE_Malloc(size as _, hint.into()) as _ }
    }

    fn dealloc_impl(ptr: *mut u8) {
        unsafe { crate::wrapper::raw::TEE_Free(ptr as _) }
    }

    fn realloc_impl(ptr: *const u8, new_size: usize) -> *mut u8 {
        //TEE_Realloc will deallocate the old memory if the allocation had to be moved
        // this also means the previously allocated memory was moved
        unsafe { crate::wrapper::raw::TEE_Realloc(ptr as _, new_size as _) as _ }
    }
}

unsafe impl core::alloc::GlobalAlloc for TEEAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        Self::alloc_impl(size, AllocHint::Unshareable)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        Self::dealloc_impl(ptr)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();

        Self::alloc_impl(size, AllocHint::UnshareableZeroed)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: the caller must ensure that the `new_size` does not overflow.
        // `layout.align()` comes from a `Layout` and is thus guaranteed to be valid.
        let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());
        Self::realloc_impl(ptr as _, new_layout.size())
    }
}
