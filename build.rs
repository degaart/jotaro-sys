use bindgen::MacroTypeVariation;
use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=minizip-src");

    let openssl_root = env::var("DEP_OPENSSL_ROOT").unwrap();
    let out_path = Config::new("minizip-src")
        .define("MZ_WZAES", "ON")
        .define("MZ_SIGNING", "ON")
        .define("MZ_BZIP2", "OFF")
        .define("MZ_LZMA", "OFF")
        .define("MZ_ZSTD", "OFF")
        .define("MZ_LIBCOMP", "OFF")
        .define("OPENSSL_ROOT_DIR", &openssl_root)
        .define("OPENSSL_USE_STATIC_LIBS", "ON")
        .build();

    let lib_dir = out_path.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=minizip");
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=z");

    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "".to_string()) == "macos" {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
    }

    println!("cargo:rerun-if-changed=minizip.h");
    let bindings = bindgen::builder()
        .header("minizip.h")
        .clang_arg(format!("-I{}", out_path.join("include").display()))
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .ctypes_prefix("libc")
        .allowlist_function("mz_.*")
        .allowlist_type("mz_.*")
        .allowlist_var("MZ_.*")
        .blocklist_function("mz_os_.*")
        .blocklist_type(
            "DIR|.*_mutex_t|dirent|_telldir|tm|__uint8_t|__uint16_t|__uint64_t|time_t|wchar_t",
        )
        .raw_line("use libc::{tm,time_t};")
        .generate()
        .unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_dir.join("bindings.rs")).unwrap();
}
