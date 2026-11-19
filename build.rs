use std::env;
use std::path::Path;
use std::process::Command;

fn warn(title: &str, details: &str, fix: &str) {
    println!("cargo::warning={title}");
    println!("cargo::warning=\r\x1b[2K \x1b[1;34m|\x1b[0m       {details}");
    println!("cargo::warning=\r\x1b[2K \x1b[1;32m|\x1b[0m  Fix: {fix}");
}

fn main() {
    println!("cargo::rerun-if-env-changed=GSTREAMER_1_0_ROOT_ANDROID");
    println!("cargo::rerun-if-env-changed=GSTREAMER_ROOT_ANDROID");
    println!("cargo::rerun-if-env-changed=CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER");

    let target = env::var("TARGET").unwrap_or_default();
    if !target.contains("android") {
        return;
    }

    let gst_root = env::var("GSTREAMER_1_0_ROOT_ANDROID")
        .map(|val| ("GSTREAMER_1_0_ROOT_ANDROID", val))
        .or_else(|_| env::var("GSTREAMER_ROOT_ANDROID").map(|val| ("GSTREAMER_ROOT_ANDROID", val)));

    match gst_root {
        Ok((var_name, root)) => {
            let lib_dir = Path::new(&root).join("arm64/lib");
            if !lib_dir.is_dir() {
                warn(
                    &format!("{var_name} points to a missing directory."),
                    &format!("Expected GStreamer libraries at `{}`, which does not exist.", lib_dir.display()),
                    &format!("Verify your GStreamer Android SDK installation and update `{var_name}`."),
                );
            }
            println!("cargo::rustc-link-search=native={}", lib_dir.display());
        }
        Err(_) => {
            warn(
                "GSTREAMER_1_0_ROOT_ANDROID and GSTREAMER_ROOT_ANDROID are not set.",
                "This may cause the build to fail when linking GStreamer static libraries.",
                "Set `GSTREAMER_1_0_ROOT_ANDROID` or `GSTREAMER_ROOT_ANDROID` to your GStreamer Android SDK path.",
            );
        }
    }

    println!("cargo::rustc-link-lib=static=pcre2-8");
    println!("cargo::rustc-link-lib=static=iconv");
    println!("cargo::rustc-link-lib=static=ffi");
    println!("cargo::rustc-link-lib=static=gmodule-2.0");
    println!("cargo::rustc-link-lib=static=orc-0.4");

    let linker = env::var("CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER")
        .unwrap_or_else(|_| "aarch64-linux-android26-clang".to_string());

    let builtin_lib = "libclang_rt.builtins-aarch64-android.a";

    match Command::new(&linker)
        .arg(format!("--print-file-name={builtin_lib}"))
        .output()
    {
        Ok(output) if output.status.success() => {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let path = Path::new(&path_str);

            // Clang echoes back the bare filename without a parent directory if it cannot find the file
            if path_str != builtin_lib && path.exists() {
                if let Some(dir) = path.parent() {
                    println!("cargo::rustc-link-search=native={}", dir.display());
                }
                println!("cargo::rustc-link-lib=static=clang_rt.builtins-aarch64-android");
            } else {
                warn(
                    &format!("Could not locate `{builtin_lib}` via `{linker}`."),
                    "Linking may fail with undefined compiler runtime symbols (such as `__aarch64_ldadd4_acq_rel`).",
                    "Ensure your Android NDK installation includes the clang runtime builtins for API 26+.",
                );
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn(
                &format!("Android linker `{linker}` exited with status {}.", output.status),
                stderr.lines().next().unwrap_or("Failed to query clang runtime library path."),
                "Check `CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER` or your Android NDK toolchain setup.",
            );
        }
        Err(err) => {
            warn(
                &format!("Failed to execute Android linker `{linker}`: {err}."),
                "Could not locate `libclang_rt.builtins-aarch64-android.a` for linking.",
                "Add the Android NDK toolchain `bin` directory to your `PATH` or set `CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER`.",
            );
        }
    }
}
