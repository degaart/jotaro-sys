use bindgen::MacroTypeVariation;
use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=minizip-src");

    let out_path = Config::new("minizip-src")
        .define("MZ_WZAES", "ON")
        .define("MZ_SIGNING", "ON")
        .define("MZ_BZIP2", "OFF")
        .define("MZ_LZMA", "OFF")
        .define("MZ_ZSTD", "OFF")
        .define("MZ_LIBCOMP", "OFF")
        .build();
    let lib_dir = out_path.join("lib");

    env::set_var(
        "PKG_CONFIG_PATH",
        lib_dir.join("pkgconfig").to_str().unwrap(),
    );
    pkg_config::Config::new()
        .statik(true)
        .probe("minizip")
        .unwrap();

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
