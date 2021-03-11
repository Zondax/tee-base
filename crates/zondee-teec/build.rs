use std::env;

fn main() {
    let tr = env::var("TEEC_ROOT").expect("Set TEEC_ROOT environment variable");
    println!("cargo:rerun-if-changed=teec.h");
    println!("cargo:rustc-link-lib=teec");
    println!("cargo:rustc-link-search={}/lib", tr);
}
