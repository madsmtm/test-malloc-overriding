# Test system malloc overriding when using dylibs

Test the Rust crates `mimalloc` and `tikv-jemallocator` and their ability to override the system allocator.

Check out [the GitHub Actions runs](https://github.com/madsmtm/test-malloc-overriding/actions) for details on the failures. Currently macOS works, I am unsure about Linux.

See also:
- [My jemallocator PR](https://github.com/tikv/jemallocator/pull/109).
- [My mimalloc PR](https://github.com/purpleprotocol/mimalloc_rust/pull/146).

This only tests when Rust invokes the linker - this works differently when that isn't the case, since then Rust won't get a chance to insert it's `symbols.o` trick (which is what this repository tests).


## Test unused

Test that the allocator is overidden, even when the main binary doesn't allocate.

This is important in case allocator symbols are looked up using `dlsym`.

```sh
cargo run -p test-unused --features mimalloc
cargo run -p test-unused --features jemalloc
```


## Test dylib

Test that the allocator in linked dynamic libraries are also overidden.

This is important to ensure that when calling into a dynamic library that e.g. returns a pointer that it guarantees is allocated with `libc::malloc`, we can free it with `libc::free`.

```sh
cargo run -p test-dylib --features mimalloc
cargo run -p test-dylib --features jemalloc

cargo run -p test-dylib --features dylib-mimalloc,dylib-mentions
cargo run -p test-dylib --features dylib-jemalloc,dylib-mentions # Works on current, but doesn't override the allocator

cargo run -p test-dylib --features dylib-mimalloc # Works on current, but doesn't override the allocator
cargo run -p test-dylib --features dylib-jemalloc # Works on current, but doesn't override the allocator
```
