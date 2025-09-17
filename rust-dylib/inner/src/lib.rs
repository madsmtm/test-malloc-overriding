#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

use core::ffi::{c_char, c_void};

#[link(name = "c")]
unsafe extern "C" {}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { libc::abort() }
}

#[lang = "eh_personality"]
unsafe extern "C" fn rust_eh_personality() -> ! {
    unsafe { libc::abort() }
}

// MARK: Actual code below

#[cfg(feature = "jemalloc")]
use jemalloc as _;
#[cfg(feature = "mimalloc")]
use mimalloc as _;

#[unsafe(no_mangle)]
pub extern "C" fn rust_dylib_lookup_malloc_address() -> *const c_char {
    #[cfg(all(feature = "jemalloc", feature = "mentions"))]
    let _ = unsafe { jemalloc::malloc(42) };
    #[cfg(all(feature = "mimalloc", feature = "mentions"))]
    let _ = unsafe { mimalloc::mi_malloc(42) };
    dl_utils::lookup_malloc_address().as_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_dylib_malloc(size: usize) -> *mut c_void {
    unsafe { libc::malloc(size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_dylib_free(ptr: *mut c_void) {
    unsafe { libc::free(ptr) }
}
