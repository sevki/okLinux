use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let vmlinux = env::var("VMLINUX").unwrap_or_else(|_| {
        // Default: look for kernel/vmlinux relative to the workspace root
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        manifest_dir
            .parent()
            .unwrap()
            .join("kernel/vmlinux")
            .to_string_lossy()
            .into_owned()
    });

    println!("cargo::rerun-if-changed={}", vmlinux);
    println!("cargo::rerun-if-changed=build.rs");

    let output = Command::new(
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .join("scrape-syscalls/target/debug/scrape-syscalls"),
    )
    .arg("--rust")
    .arg(&vmlinux)
    .output()
    .expect("failed to run scrape-syscalls — build it first with `cargo build -p scrape-syscalls`");

    if !output.status.success() {
        panic!(
            "scrape-syscalls failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let syscalls_rs = manifest_dir.join("src/syscalls.rs");
    std::fs::write(&syscalls_rs, &output.stdout).expect("failed to write src/syscalls.rs");
}
