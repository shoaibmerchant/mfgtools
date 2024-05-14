#![allow(improper_ctypes)]

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=../msvc/x64/Release");
    println!("cargo:rustc-link-search=../libusb/build/v143/x64/Release/dll");
    println!("cargo:rustc-link-search=../msvc/bin/x64_Release");
    // println!("cargo:rustc-link-search=C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.39.33519/lib/x64/msvcrt.lib");
    // println!("cargo:rustc-link-search=C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Redist/MSVC/14.38.33135/x64/Microsoft.VC143.CRT");
    // println!("cargo:rustc-link-search=C:/Program Files (x86)/Windows Kits/10/Lib/10.0.22621.0/um/x64");
    // println!("cargo:rustc-link-search=C:/Program Files (x86)/Windows Kits/10/Lib/10.0.22621.0/ucrt/x64");

    // println!("cargo:rustc-link-search=/usr/local/lib/");
    // println!("cargo:rustc-link-search=/usr/lib/");
    // println!("cargo:rustc-link-search=/opt/homebrew/lib/");
    // println!("cargo:rustc-link-search=/opt/homebrew/opt/gcc");
    // println!("cargo:rustc-link-search=/opt/homebrew/opt/gcc@13");
    // println!("cargo:rustc-link-search=/opt/homebrew/Cellar/gcc/13.2.0/lib/gcc/13/");

    println!("cargo:rustc-link-lib=dylib=libuuu");
    println!("cargo:rustc-link-lib=dylib=libzstd_static");
    println!("cargo:rustc-link-lib=dylib=libusb-1.0");
    println!("cargo:rustc-link-lib=dylib=bzip2");
    println!("cargo:rustc-link-lib=dylib=tinyxml2");
    println!("cargo:rustc-link-lib=dylib=zlib");

    // println!("cargo:rustc-link-lib=dylib=libusb-1.0");
    // println!("cargo:rustc-link-lib=dylib=stdc++");
    // println!("cargo:rustc-link-lib=dylib=gcc");

    // println!("cargo:rustc-link-lib=dylib=usb-1.0");
    // println!("cargo:rustc-link-lib=dylib=crypto");
    // println!("cargo:rustc-link-lib=dylib=c");
    // println!("cargo:rustc-link-lib=dylib=m");
    // println!("cargo:rustc-link-lib=dylib=z");
    // println!("cargo:rustc-link-lib=dylib=zstd");
    // println!("cargo:rustc-link-lib=dylib=tinyxml2");
    // println!("cargo:rustc-link-lib=dylib=bz2");
    // println!("cargo:rustc-link-lib=dylib=ssl");

    println!("cargo:rerun-if-changed=wrapper.h");

    // env:LIBUUU_RS_CLANG_ARGS
    // linux-amd64: -I../libuuu -I/usr/include -I/usr/lib/gcc/x86_64-linux-gnu/11/include -I/usr/include/libusb-1.0
    // linux-arm64:
    // macos-arm64: -x c++ -I../libuuu -I/opt/homebrew/include -I/opt/homebrew/include/libusb-1.0
    // windows: -I../libuuu

    let clang_args_env = env::var("LIBUUU_RS_CLANG_ARGS").unwrap_or("".to_string()).to_string();
    let clang_args: Vec<&str> = clang_args_env.split(' ').collect();

    println!("CLANG_ARG {:?}", clang_args);

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Icpp")
        .clang_args(&["-x", "c++"])
        .clang_arg("-std=c++11")
        .clang_arg("-I../libuuu")
        // .clang_arg("-Lusb-1.0")
        // .clang_arg("-MDd")
        // .clang_arg("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.22621.0/ucrt")
        // .clang_arg("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.22621.0/um")
        // .clang_arg("-IC:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.39.33519/include")
        // .clang_arg("-s ERROR_ON_UNDEFINED_SYMBOLS=0")
        // .clang_args(clang_args)
        // .clang_arg("-I/usr/include")
        // .clang_arg("-I/usr/lib/gcc/x86_64-linux-gnu/11/include")
        // .clang_arg("-I/usr/include/libusb-1.0")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
