#![no_std]

use core::ffi::{CStr, c_char, c_void};

unsafe extern "C" {
    pub unsafe fn c_dylib_malloc(size: usize) -> *mut c_void;
    pub unsafe fn c_dylib_free(ptr: *mut c_void);
}

pub fn c_dylib_lookup_malloc_address() -> &'static CStr {
    unsafe extern "C" {
        safe fn c_dylib_lookup_malloc_address() -> *const c_char;
    }

    unsafe { CStr::from_ptr(c_dylib_lookup_malloc_address()) }
}
