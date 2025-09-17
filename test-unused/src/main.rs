//! A `#![no_std]` binary.
//!
//! The standard library runtime allocates before `main`, which interferes
//! with this test, since that makes the `malloc`/`free` symbols used.
#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(lang_items)]

use core::ffi::{c_char, c_int};
use core::panic::PanicInfo;

#[link(name = "c")]
unsafe extern "C" {}

#[panic_handler]
fn panic_handler(_info: &PanicInfo<'_>) -> ! {
    unsafe { libc::abort() }
}

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() -> i32 {
    unsafe { libc::abort() }
}

// MARK: Actual code below

#[cfg(feature = "jemalloc")]
use jemalloc as _;
#[cfg(feature = "mimalloc")]
use mimalloc as _;

#[unsafe(no_mangle)]
extern "C" fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    // Test whether the symbols are available - that's a proxy for testing if
    // the linker were aware of the allocator, and overrode it.

    #[cfg(feature = "mimalloc")]
    assert!(!unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"mi_malloc".as_ptr()) }.is_null());

    #[cfg(feature = "jemalloc")]
    assert!(!unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"_rjem_malloc".as_ptr()) }.is_null());

    0
}
