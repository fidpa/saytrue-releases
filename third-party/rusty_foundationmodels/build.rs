#![allow(clippy::disallowed_methods)]

use std::process::Command;

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "macos" && target_os != "ios" {
        return;
    }
    // Saytrue-Patch: iOS (Gerät und Simulator) mit derselben bridge.swift. Mindestversion
    // iOS 26.0 wie das Xcode-Projekt; das Archiv landet über die staticlib in libapp.a, das
    // Framework zieht der Swift-Autolink nach.
    let is_ios_sim = std::env::var("TARGET").unwrap_or_default().ends_with("-sim");
    let (sdk, swift_target) = match (target_os.as_str(), is_ios_sim) {
        ("ios", true) => ("iphonesimulator", "arm64-apple-ios26.0-simulator"),
        ("ios", false) => ("iphoneos", "arm64-apple-ios26.0"),
        _ => ("macosx", "arm64-apple-macos15.0"),
    };

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let swift_file = format!("{manifest_dir}/bridge.swift");
    let lib_path = format!("{out_dir}/librusty_fm_bridge.a");

    println!("cargo:rerun-if-changed=bridge.swift");
    // Declare the custom cfg flag so rustc doesn't warn about it on any host.
    println!("cargo::rustc-check-cfg=cfg(foundation_models_bridge)");

    let status = Command::new("xcrun")
        .args([
            "--sdk",
            sdk,
            "swiftc",
            "-emit-library",
            "-static",
            "-parse-as-library",
            "-module-name",
            "RustyFMBridge",
            // Minimum deployment target; FoundationModels is guarded at runtime with
            // #available so the binary still runs on older macOS without crashing.
            "-target",
            swift_target,
            "-o",
            &lib_path,
            &swift_file,
        ])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("cargo:rustc-cfg=foundation_models_bridge");
            println!("cargo:rustc-link-search=native={out_dir}");
            println!("cargo:rustc-link-lib=static=rusty_fm_bridge");
            if target_os == "macos" {
                // FoundationModels is weak-linked so the binary doesn't crash on
                // macOS < 26 where the framework doesn't exist yet.
                println!("cargo:rustc-link-arg=-Wl,-weak_framework,FoundationModels");
                // Required for Swift concurrency runtime.
                println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
            }
        }
        Ok(_) => {
            println!(
                "cargo::warning=rusty_foundationmodels: Swift bridge compilation failed \
                 (requires Xcode with macOS 26+ SDK). All APIs will return \
                 Err(Error::Unavailable) at runtime."
            );
        }
        Err(e) => {
            println!(
                "cargo::warning=rusty_foundationmodels: xcrun not found, \
                 skipping Swift bridge: {e}"
            );
        }
    }
}
