use std::env;

fn main() {
    let ur = env::var("UTEE_ROOT").expect("Set UTEE_ROOT environment variable");
    println!("cargo:rerun-if-changed=os.h");
    println!("cargo:rustc-link-lib=static=mbedtls");
    println!("cargo:rustc-link-lib=static=utee");
    println!("cargo:rustc-link-lib=static=utils");
    println!("cargo:rustc-link-search={}/lib", ur);
}
