fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("android") {
        if let Ok(root) = std::env::var("GSTREAMER_1_0_ROOT_ANDROID") {
            println!("cargo:rustc-link-search=native={root}/arm64/lib");
        }
        println!("cargo:rustc-link-lib=static=pcre2-8");
        println!("cargo:rustc-link-lib=static=iconv");
        println!("cargo:rustc-link-lib=static=ffi");
        println!("cargo:rustc-link-lib=static=gmodule-2.0");
        println!("cargo:rustc-link-lib=static=orc-0.4");

        if let Ok(linker) = std::env::var("CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER") {
            if let Ok(output) = std::process::Command::new(linker)
                .arg("--print-file-name=libclang_rt.builtins-aarch64-android.a")
                .output()
            {
                if output.status.success() {
                    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if let Some(dir) = std::path::Path::new(&path).parent() {
                        println!("cargo:rustc-link-search=native={}", dir.display());
                    }
                    println!("cargo:rustc-link-lib=static=clang_rt.builtins-aarch64-android");
                }
            }
        }
    }
}
