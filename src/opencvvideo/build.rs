use cxx_build::CFG;
use std::path::Path;

fn main() {
    CFG.exported_header_dirs.push(Path::new("D:/Programs/opencv/Release/include"));
    //CFG.exported_header_prefixes.push("D:/Programs/opencv/Release/lib");
    //CFG.exported_header_links.push("opencv_world490");
    //CFG.exported_header_links.push("opencv_xfeatures2");
    //CFG.exported_header_links.push("opencv_imgproc490");

    cxx_build::bridge("examples/example_13-01.rs")
        .file("src/opencv.cc")
        .std("c++20")
        .compile("cxxbridge-opencv");

    println!("cargo:rerun-if-changed=src/examples/example_13-01.rs");
    println!("cargo:rerun-if-changed=src/opencv.cc");
    println!("cargo:rerun-if-changed=include/opencv.h");
}
