#![allow(improper_ctypes)]

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=../build/libuuu/");
    println!("cargo:rustc-link-search=/lib/aarch64-linux-gnu/");
    println!("cargo:rustc-link-search=/usr/lib/aarch64-linux-gnu/");

    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=usb-1.0");
    println!("cargo:rustc-link-lib=dylib=crypto");
    println!("cargo:rustc-link-lib=dylib=z");
    println!("cargo:rustc-link-lib=dylib=zstd");
    println!("cargo:rustc-link-lib=dylib=bz2");
    println!("cargo:rustc-link-lib=dylib=ssl");
    println!("cargo:rustc-link-lib=static=uuc_s");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I../libuuu")
        .clang_arg("-I/usr/include")
        .clang_arg("-I/usr/include/libusb-1.0")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
