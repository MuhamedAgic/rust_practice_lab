use std::env;
use cxx_build;

fn main() {
    cxx_build::bridge("src/lib.rs")
        .compile("rust_practice_lab");

    // Print the target directory for debugging
    println!("cargo:warning=Target directory: {}", env::var("OUT_DIR").unwrap());
}