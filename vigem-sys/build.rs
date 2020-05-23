#[cfg(target_arch = "x86_64")]
pub const LIB_NAME: &str = "VigemClient_x64";

#[cfg(target_arch = "x86")]
pub const LIB_NAME: &str = "VigemClient_x86";

use std::fs;

fn main() {
    let project_dir = std::env::var("OUT_DIR").unwrap();
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let first_path = format!("{}\\libs\\{}.lib", root_dir, LIB_NAME);
    let second_path = format!("{}\\{}.lib", project_dir, LIB_NAME);

    let result = fs::copy(&first_path, &second_path);
    if result.is_err(){
        println!("cargo:warning=Failed to copy file. Hi, docs.rs!");
    }

    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-link-search=static={}", format!("{}\\", project_dir));
    println!("cargo:rustc-link-lib=setupapi");

    println!("cargo:rustc-link-lib={}", LIB_NAME);
}
