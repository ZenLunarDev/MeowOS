use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use uefi::boot::MemoryType;

#[global_allocator]
static ALLOCATOR: UefiAllocator = UefiAllocator;

pub struct UefiAllocator;

unsafe impl GlobalAlloc for UefiAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match uefi::boot::allocate_pool(MemoryType::LOADER_DATA, layout.size()) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => core::ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        if !ptr.is_null() {
            let _ = uefi::boot::free_pool(NonNull::new_unchecked(ptr));
        }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = self.alloc(layout);
        if !ptr.is_null() {
            core::ptr::write_bytes(ptr, 0, layout.size());
        }
        ptr
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        if ptr.is_null() {
            return self.alloc(Layout::from_size_align_unchecked(new_size, _layout.align()));
        }
        let new_ptr = self.alloc(Layout::from_size_align_unchecked(new_size, _layout.align()));
        if !new_ptr.is_null() {
            core::ptr::copy_nonoverlapping(ptr, new_ptr, _layout.size().min(new_size));
            self.dealloc(ptr, _layout);
        }
        new_ptr
    }
}
