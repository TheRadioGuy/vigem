#[cfg(target_arch = "x86_64")]
pub const DLL_NAME: &str = "VigemClient_x64";

#[cfg(target_arch = "x86")]
pub const DLL_NAME: &str = "VigemClient_x86";

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rustc-link-search=libs");
    println!("cargo:rustc-link-lib=setupapi");
    println!("cargo:rustc-link-lib={}", DLL_NAME);
}