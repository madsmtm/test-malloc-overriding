#![no_std]

use core::ffi::{CStr, c_char, c_void};

#[link(name = "rust_dylib_inner")]
unsafe extern "C" {
    pub unsafe fn rust_dylib_malloc(size: usize) -> *mut c_void;
    pub unsafe fn rust_dylib_free(ptr: *mut c_void);
}

pub fn rust_dylib_lookup_malloc_address() -> &'static CStr {
    unsafe extern "C" {
        safe fn rust_dylib_lookup_malloc_address() -> *const c_char;
    }

    unsafe { CStr::from_ptr(rust_dylib_lookup_malloc_address()) }
}
