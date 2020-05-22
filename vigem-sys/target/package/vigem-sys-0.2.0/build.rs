#[cfg(target_arch = "x86_64")]
pub const LIB_NAME: &str = "VigemClient_x64";

#[cfg(target_arch = "x86")]
pub const LIB_NAME: &str = "VigemClient_x86";

fn main() {
    println!("cargo:rerun-if-changed=libs/VigemClient_x64.lib");
    println!("cargo:rustc-link-search=./libs");
    println!("cargo:rustc-link-lib=setupapi");
    println!("cargo:rustc-link-lib={}", LIB_NAME);
}
