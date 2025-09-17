# Test system malloc overriding when using dylibs


## Test unused

```sh
cargo run -p test-unused --features mimalloc
cargo run -p test-unused --features jemalloc
```

## Test dylib

```sh
cargo run -p test-dylib --features mimalloc
cargo run -p test-dylib --features jemalloc

cargo run -p test-dylib --features dylib-mimalloc,dylib-mentions
cargo run -p test-dylib --features dylib-jemalloc,dylib-mentions

cargo run -p test-dylib --features dylib-mimalloc
cargo run -p test-dylib --features dylib-jemalloc
```
