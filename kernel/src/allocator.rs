use core::alloc::{GlobalAlloc, Layout};
use uefi::boot::{AllocateType, MemoryType};

#[global_allocator]
static ALLOCATOR: UefiAllocator = UefiAllocator;

pub struct UefiAllocator;

unsafe impl GlobalAlloc for UefiAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = uefi::boot::allocate_pool(MemoryType::LOADER_DATA, layout.size())
            .unwrap_or(0 as *mut u8);
        ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let _ = uefi::boot::free_pool(ptr as *mut _);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = uefi::boot::allocate_pool(MemoryType::LOADER_DATA, layout.size())
            .unwrap_or(0 as *mut u8);
        if !ptr.is_null() {
            core::ptr::write_bytes(ptr, 0, layout.size());
        }
        ptr as *mut u8
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = uefi::boot::allocate_pool(MemoryType::LOADER_DATA, new_size)
            .unwrap_or(0 as *mut u8);
        if !new_ptr.is_null() {
            core::ptr::copy_nonoverlapping(ptr, new_ptr, _layout.size().min(new_size));
            uefi::boot::free_pool(ptr as *mut _);
        }
        new_ptr as *mut u8
    }
}
