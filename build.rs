use std::process::Command;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(&manifest_dir)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();
    let hash = hash.trim();

    let date = Command::new("git")
        .args(["log", "-1", "--format=%cd", "--date=short"])
        .current_dir(&manifest_dir)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();
    let date = date.trim();

    if !hash.is_empty() {
        println!("cargo:rustc-env=MD_BUILD_INFO={hash} ({date})");
    } else {
        println!("cargo:rustc-env=MD_BUILD_INFO=dev");
    }

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/");
    println!("cargo:rerun-if-changed=build.rs");
}
