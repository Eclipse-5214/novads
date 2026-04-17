use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    let libds_path = project_dir.join("libds");
    let libds_inc = libds_path.join("include");
    let socky_path = libds_path.join("lib/Socky/src");

    compile_libds(&libds_path, &socky_path);

    let header_file = libds_inc.join("LibDS.h");
    let mut builder = bindgen::Builder::default()
        .header(header_file.to_str().unwrap())
        .clang_arg(format!("-I{}", libds_inc.display()))
        .clang_arg(format!("-I{}", socky_path.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    if target.contains("android") {
        builder = configure_android_bindgen(builder, &target);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");
    bindings.write_to_file(out_dir.join("bindings.rs")).expect("Couldn't write bindings!");

    tauri_build::build();
}

fn compile_libds(libds_path: &Path, socky_path: &Path) {
    println!("cargo:rerun-if-changed={}", libds_path.join("src").display());
    println!("cargo:rerun-if-changed={}", libds_path.join("include").display());
    println!("cargo:rerun-if-changed={}", socky_path.display());

    let src = libds_path.join("src");
    let mut build = cc::Build::new();

    let core_files = [
        "client.c", "config.c", "crc32.c", "events.c", "init.c",
        "joysticks.c", "protocols.c", "socket.c", "timer.c",
        "utils.c", "string.c", "array.c", "queue.c",
    ];

    let protocols = [
        "frc_2014.c", "frc_2015.c", "frc_2016.c", "frc_2020.c",
    ];

    build
        .include(libds_path.join("include"))
        .include(socky_path)
        .include(src.join("protocols"));

    for file in &core_files {
        build.file(src.join(file));
    }

    for file in &protocols {
        build.file(src.join("protocols").join(file));
    }

    build.file(socky_path.join("socky.c"));
    build.warnings(false).compile("ds");
}

fn configure_android_bindgen(builder: bindgen::Builder, target: &str) -> bindgen::Builder {
    let ndk_home = env::var("NDK_HOME")
        .or_else(|_| env::var("ANDROID_NDK_HOME"))
        .expect("NDK_HOME or ANDROID_NDK_HOME must be set for Android builds");

    let host_os = if cfg!(target_os = "macos") { "darwin-x86_64" } 
                  else if cfg!(target_os = "windows") { "windows-x86_64" } 
                  else { "linux-x86_64" };
    
    let sysroot = PathBuf::from(&ndk_home)
        .join("toolchains/llvm/prebuilt")
        .join(host_os)
        .join("sysroot");

    let target_include_dir = match target {
        "aarch64-linux-android" => "aarch64-linux-android",
        "armv7-linux-androideabi" => "arm-linux-androideabi",
        "i686-linux-android" => "i686-linux-android",
        "x86_64-linux-android" => "x86_64-linux-android",
        _ => "aarch64-linux-android",
    };

    builder
        .clang_arg(format!("--sysroot={}", sysroot.display()))
        .clang_arg(format!("-I{}", sysroot.join("usr/include").join(target_include_dir).display()))
        .clang_arg(format!("-I{}", sysroot.join("usr/include").display()))
}