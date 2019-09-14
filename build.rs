// extern crate cpp_build;

fn main() {
    let include_path = "/Users/ppamorim/Repository/rust-cpp/src/lib";
    let lib_path = "/Users/ppamorim/Repository/rust-cpp/src/lib";
    cpp_build::Config::new().include(include_path).build("src/lib.rs");
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=static=sum");
}