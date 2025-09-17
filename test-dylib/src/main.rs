//! Test that the `malloc` and `free` symbols are interoperable, even across a
//! dylib boundary.

use c_dylib::{c_dylib_free, c_dylib_lookup_malloc_address, c_dylib_malloc};
use dl_utils::lookup_malloc_address;
use rust_dylib::{rust_dylib_free, rust_dylib_lookup_malloc_address, rust_dylib_malloc};

fn main() {
    // Check that pointers created with `malloc` in a dylib dependency can be
    // free'd with `free` here.
    let mallocs = &[
        libc::malloc,
        c_dylib_malloc,
        rust_dylib_malloc,
        #[cfg(feature = "mimalloc")]
        mimalloc::mi_malloc,
        #[cfg(feature = "jemalloc")]
        jemalloc::malloc,
    ];
    let frees = &[
        libc::free,
        c_dylib_free,
        rust_dylib_free,
        #[cfg(feature = "mimalloc")]
        mimalloc::mi_free,
        #[cfg(feature = "jemalloc")]
        jemalloc::free,
    ];

    for malloc in mallocs {
        for free in frees {
            let ptr = unsafe { malloc(42) };
            unsafe { free(ptr) };
        }
    }

    // Extra check that the symbols were actually from the same place.
    let c = c_dylib_lookup_malloc_address();
    let rust = rust_dylib_lookup_malloc_address();
    let here = lookup_malloc_address();

    if cfg!(target_vendor = "apple") {
        // macOS / Mach-O symbols are not overidden in dependencies, they are
        // hooked into with `zone_register`.
        assert_eq!(c, c"/usr/lib/system/libsystem_malloc.dylib");
        #[cfg(not(feature = "dylib-mimalloc"))]
        assert_eq!(rust, c"/usr/lib/system/libsystem_malloc.dylib");

        // TODO: Mimalloc overrides the symbols themselves; is this okay?
        #[cfg(not(any(feature = "mimalloc", feature = "dylib-mimalloc")))]
        assert_eq!(here, c"/usr/lib/system/libsystem_malloc.dylib");
    } else {
        assert_eq!(c, here);
        assert_eq!(rust, here);
    }
}
