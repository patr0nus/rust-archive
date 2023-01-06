use std::env;
use std::path::PathBuf;

fn main() {
    let cmake_out = cmake::Config::new("libarchive")
        .build_target("archive_static")
        .define("ENABLE_BZip2", "ON")
        .define("CMAKE_REQUIRE_FIND_PACKAGE_BZip2", "TRUE")
        .define("ENABLE_LIBXML2", "ON")
        .define("CMAKE_REQUIRE_FIND_PACKAGE_LibXml2", "TRUE")
        .define("ENABLE_LZ4", "ON")
        .define("CMAKE_REQUIRE_FIND_PACKAGE_lz4", "TRUE")
        .define("ENABLE_LZMA", "ON")
        .define("CMAKE_REQUIRE_FIND_PACKAGE_LibLZMA", "TRUE")
        .define("ENABLE_OPENSSL", "ON")
        .define("CMAKE_REQUIRE_FIND_PACKAGE_OpenSSL", "TRUE")
        .define("ENABLE_ZLIB", "ON")
        .define("CMAKE_REQUIRE_FIND_PACKAGE_zlib", "TRUE")
        .define("ENABLE_ZSTD", "ON")
        .define("ENABLE_LZO", "ON")
        .define("ENABLE_PCREPOSIX", "OFF")
        .define("POSIX_REGEX_LIB", "NONE")
        .define("ENABLE_NETTLE", "OFF")
        .define("ENABLE_EXPAT", "OFF")
        .define("ENABLE_LIBGCC", "OFF")
        .define("ENABLE_LIBB2", "OFF")
        .define("ENABLE_TEST", "OFF")
        .build();
    println!("cargo:rustc-link-search=native={}", cmake_out.join("build").join("libarchive").display());
    println!("cargo:rustc-link-lib=static=archive");

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        println!("cargo:rustc-link-lib=User32");
        println!("cargo:rustc-link-lib=Crypt32");
    }

    println!("cargo:rerun-if-changed=wrapper.h");
    let mut bindgen_builder = bindgen::builder()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("archive_.*")
        .allowlist_var("ARCHIVE_.*")
        .blocklist_type("FILE")
        .blocklist_type("timespec")
        .blocklist_type("stat")
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .clang_args(["-I", &(env::var("CARGO_MANIFEST_DIR").unwrap() + "/libarchive/libarchive")])
        .raw_line("use libc::{stat, FILE};");

    let bindings = bindgen_builder.generate().unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
