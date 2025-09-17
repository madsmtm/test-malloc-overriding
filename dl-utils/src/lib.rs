#![no_std]
use core::ffi::{CStr, c_void};

pub fn lookup_malloc_address() -> &'static CStr {
    unsafe {
        let mut info: libc::Dl_info = core::mem::zeroed();
        let fnptr: unsafe extern "C" fn(libc::size_t) -> *mut c_void = libc::malloc;
        let fnptr = fnptr as *const c_void;
        if libc::dladdr(fnptr, &mut info) == 0 {
            libc::printf(b"failed finding `malloc`\n\0".as_ptr().cast());
            libc::abort();
        }
        CStr::from_ptr(info.dli_fname)
    }
}
