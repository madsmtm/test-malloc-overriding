//! Add the `target/debug/deps` dir to make linking work.
use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/dep.c");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let deps_dir = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("deps");

    println!("cargo:rustc-link-search=native={}", deps_dir.display());
}
