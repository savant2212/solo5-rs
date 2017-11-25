// implement rust allocation over solo5 malloc interface

use solo5_malloc;
use solo5_realloc;
use solo5_free;

extern crate alloc;

use self::alloc::allocator::{Alloc, AllocErr, Layout};
use core::ptr;
pub struct Solo5Allocator;

unsafe impl<'a> Alloc for &'a Solo5Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let ptr = solo5_malloc(layout.size());
        if ptr ==  ptr::null_mut() {
            Err(AllocErr::Exhausted{request : layout})
        } else {
            Ok(ptr)
        }
    }
    #[allow(unused_variables)]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        if ptr != ptr::null_mut() {
            solo5_free(ptr);
        }
    }
}

#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe { solo5_malloc(size) as *mut u8 }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    unsafe { solo5_free(ptr) }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,
                                _align: usize) -> *mut u8 {
    unsafe {
        solo5_realloc(ptr, size) as *mut u8
    }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size // this api is not supported by libc
}
/*
#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
*/
