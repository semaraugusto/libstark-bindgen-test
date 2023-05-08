extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=RustyLibstark");

    println!("cargo:rustc-link-search=../libSTARK/bin/libstark",);
    println!("cargo:rustc-link-lib=static=stark");

    println!("cargo:rustc-link-search=../libSTARK/bin/algebralib",);
    println!("cargo:rustc-link-lib=static=algebralib");

    println!("cargo:rustc-link-search=../libSTARK/bin/fft",);
    println!("cargo:rustc-link-lib=static=FFT");

    println!("cargo:rustc-link-lib=static=RustyLibstark");
    println!("cargo:rustc-link-lib=dylib=gtest");
    println!("cargo:rustc-link-lib=dylib=gomp");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // Generating object files
    let compilation_output = std::process::Command::new("g++")
        .arg("-std=c++11")
        .arg("-O3")
        .arg("-g")
        .arg("-Wall")
        .arg("-Xpreprocessor")
        .arg("-maes")
        .arg("-msse4")
        .arg("-mtune=native")
        .arg("src/wrapper.cpp")
        .arg("../libSTARK/libstark-tests/BairWitnessChecker_UTEST.cpp")
        .arg("../libSTARK/libstark-tests/lightCircLib/lightCircGate.cpp")
        .arg("../libSTARK/libstark-tests/lightCircLib/lightCircPoly.cpp")
        .arg("../libSTARK/libstark-tests/lightCircLib/lightCircuit.cpp")
        .arg("-Iinclude")
        .arg("-I../libSTARK/libstark-tests/")
        .arg("-I../libSTARK/libstark/src")
        .arg("-I../libSTARK/algebra/algebralib/headers")
        .arg("-I../libSTARK/algebra/FFT/src")
        .arg("-c")
        .output()
        .expect("Could not spawn g++");

    if !compilation_output.status.success() {
        panic!(
            "Could not compile c++ code correctly. Error: {:#?}",
            compilation_output
        )
    }
    let lib_file = out_dir.clone().join("libRustyLibstark.a");
    // let obj_file = out_dir.clone"wrapper.o");

    // Generating library file
    let lib_gen_output = std::process::Command::new("ar")
        .arg("cr")
        .arg(lib_file.to_str().unwrap())
        .arg("wrapper.o")
        .arg("BairWitnessChecker_UTEST.o")
        .arg("lightCircGate.o")
        .arg("lightCircPoly.o")
        .arg("lightCircuit.o")
        .output()
        .expect("Could not spawn ar");

    if !lib_gen_output.status.success() {
        panic!(
            "Could not generate libRustyLibstark.a file correctly. Error: {:#?}",
            lib_gen_output
        )
    }

    let bindings = bindgen::Builder::default()
        .header("include/wrapper.hpp")
        .enable_cxx_namespaces()
        .clang_arg("-std=c++11")
        .opaque_type("std::.*")
        .allowlist_function("RustyLibstark::.*")
        // .manually_drop_union()
        .parse_callbacks(Box::new(CargoCallbacks))
        .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");
}
