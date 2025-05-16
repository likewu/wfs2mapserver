use cxx_build::CFG;

fn main() {
    cxx_build::bridge("examples/example_13-01.rs")
        .file("src/opencv.cc")
        .std("c++20")
        .compile("cxxbridge-opencv");

    println!("cargo:rerun-if-changed=src/examples/example_13-01.rs");
    println!("cargo:rerun-if-changed=src/opencv.cc");
    println!("cargo:rerun-if-changed=include/opencv.h");
}
