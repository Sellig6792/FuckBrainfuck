use std::ops::Not;

fn main() {
    //Set Binary Name
    let binary_name = match &std::env::var("TARGET").unwrap() as &str {
        "aarch64-unknown-linux-gnu" => "fuckbf-arm64",
        "i686-unknown-linux-gnu" => "fuckbf-linux-i686",
        "x86_64-unknown-linux-gnu" => "fuckbf-linux-x86_64",
        "x86_64-apple-darwin" => "fuckbf-macos",
        "i686-pc-windows-gnu" => "fuckbf-win-i686.exe",
        "x86_64-pc-windows-gnu" => "fbf-win-x86_64.exe",
        _ => "",
    };
    if binary_name.is_empty().not() {
        println!("cargo:rustc-cfg=fuckbf_updatable");
        println!("cargo:rustc-env=FUCKBF_BINARY_NAME={}", binary_name);
    }
}
