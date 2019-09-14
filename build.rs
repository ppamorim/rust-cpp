extern crate cpp_build;
use std::env::var;

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    let include_path = format!("{}/src/lib", manifest_dir);
    let lib_path = format!("{}/src/lib", manifest_dir);
    cpp_build::Config::new().include(include_path).build("src/lib.rs");
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=static=sum");
}