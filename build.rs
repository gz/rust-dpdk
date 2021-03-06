extern crate bindgen;
extern crate cc;

use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-lib=dpdk");
    cc::Build::new()
        .flag("-I/usr/local/include/dpdk")
        .file("dpdk-inlined.c")
        .compile("dpdk-inlined.a");
    let bindings = bindgen::Builder::default()
        .header("rte.h")
        .header("dpdk-inlined.h")
        .generate_inline_functions(true)
        .clang_arg("-I/usr/local/include/dpdk")
        .clang_arg("-Wno-error=implicit-function-declaration")
        .clang_arg("-fno-inline-functions")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("ffi.rs"))
        .expect("Cooudn't write bindings");
    let mut child = Command::new("perl")
        .arg("-pi")
        .arg("-e")
        .arg("s/(pub static mut per_lcore__lcore_id)/    #[thread_local] $1/")
        .arg(out_path.join("ffi.rs").as_os_str())
        .spawn()
        .expect("failed to execute perl");
    let _ = child.wait();
}
