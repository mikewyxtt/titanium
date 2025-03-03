use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() != "x86_64" {
        panic!("Error: Building for architecture {} is not supported at this time. Only x86_64 is supported.", env::var("CARGO_CFG_TARGET_ARCH").unwrap());
    }

    build_sel4();
}

fn build_sel4() {
    let workspace_root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let rust_target_dir = std::env::var("OUT_DIR").unwrap();
    let sel4_source_dir = Path::new(&workspace_root).join("src/seL4");
    let build_dir = Path::new(&rust_target_dir).join("sel4_build");
    let staging_dir = Path::new(&workspace_root).join("target/STAGING");
    println!("cargo:rerun-if-changed={}", sel4_source_dir.display());

    // Create the build directory if it doesn't exist
    if !build_dir.exists() {
        fs::create_dir_all(&build_dir).expect("Failed to create directory");
    }

    if !Path::new(&staging_dir).exists() {
        fs::create_dir_all(&staging_dir).expect("Failed to create directory");
    }

    // Construct the CMake command
    let status = Command::new("cmake")
        .arg(format!("-DCMAKE_TOOLCHAIN_FILE={}/llvm.cmake", sel4_source_dir.display()))
        .arg("-DPLATFORM=x86_64")
        .arg("-DTRIPLE=x86_64-unknown-none")
        .arg("-DSIMULATION=TRUE")
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", staging_dir.display()))
        .arg(format!("-DSEL4_CACHE_DIR={}/.sel4_cache", rust_target_dir))
        .arg("-C")
        .arg(format!("{}/configs/X64_custom.cmake", sel4_source_dir.display()))
        .arg("-G")
        .arg("Ninja")
        .arg(sel4_source_dir)
        .current_dir(&build_dir)
        .status()
        .expect("Failed to execute cmake");

    if !status.success() {
        panic!("seL4 BUILD ERROR: CMake command failed!");
    }

    // Run ninja to build seL4
    let status = Command::new("ninja")
        .current_dir(&build_dir)
        .status()
        .expect("seL4 BUILD ERROR: Failed to execute ninja");

    if !status.success() {
        panic!("Ninja build failed!");
    }

    // Run ninja to install seL4 into the staging directory
    let status = Command::new("ninja")
        .arg("install")
        .current_dir(&build_dir)
        .status()
        .expect("seL4 BUILD ERROR: Failed to execute ninja");

    if !status.success() {
        panic!("Ninja install failed!");
    }
}
