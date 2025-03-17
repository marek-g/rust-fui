use std::env;
use std::path::Path;

fn main() {
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    let target = env::var("TARGET").unwrap();
    let current_dir = env::current_dir().unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();
    let target_family = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap();

    println!("target: {}", target);
    println!("current_dir: {}", current_dir.to_string_lossy());
    println!("out_dir: {}", out_dir);
    println!("env: {}", out_dir);
    println!("target_family: {}", target_family);

    run_cbindgen();

    run_cmake(
        &current_dir.join("src/platform/qt/qt_wrapper/cpp"),
        &out_dir,
    );

    cargo_link_static(
        &Path::new(&out_dir).join("lib").to_string_lossy(),
        "qt_wrapper",
    );
    cargo_link_qt(&env, &target_family);

    generate_bindings("src/platform/qt/qt_wrapper/cpp/qt_wrapper.h", &out_dir);
}

fn run_cbindgen() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&Path::new(&out_dir).join("rust_ffi.h"));
}

fn run_cmake(src_dir: &Path, out_dir: &str) {
    let output = cmake::Config::new(src_dir)
        .define("FFI_INCLUDE_DIR", &Path::new(&out_dir))
        .build();

    println!("cmake.output: {}", output.to_string_lossy());
}

fn cargo_link_static(dir: &str, lib: &str) {
    println!("cargo:rustc-link-search={}", dir);
    println!("cargo:rustc-link-lib={}", lib);
}

fn cargo_link_qt(env: &str, target_family: &str) {
    println!("cargo:rustc-link-lib={}", "Qt6Widgets");
    println!("cargo:rustc-link-lib={}", "Qt6Gui");
    println!("cargo:rustc-link-lib={}", "Qt6Core");
    println!("cargo:rustc-link-lib={}", "Qt6OpenGL");

    if target_family == "unix" {
        println!("cargo:rustc-link-lib={}", "KF6WindowSystem");
    }

    if env != "msvc" {
        println!("cargo:rustc-link-lib=stdc++");
    }
}

fn generate_bindings(src: &str, out_dir: &str) {
    let bindings = bindgen::Builder::default()
        .header(src)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(&Path::new(out_dir).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
