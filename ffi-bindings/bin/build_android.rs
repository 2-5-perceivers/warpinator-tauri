use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs};

const LIB_NAME: &str = "libwarpinator";
const CRATE_NAME: &str = "warpinator-ffi";
const BINDGEN_BIN: &str = "bindgen";

const TARGETS: &[&str] = &["armeabi-v7a", "arm64-v8a", "x86", "x86_64"];
const FEATURES: &[&str] = &[
    "virtual_filesystem",
    "power_manager",
    "tracing_android",
    "tracing_release_max_level_debug",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let script_dir = env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::current_dir().expect("Failed to get current directory"));

    let out_dir = script_dir.join("out").join("android");
    let bindings_dir = script_dir.join("bindgen");
    let jni_libs_dir = out_dir.join("jniLibs");

    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());

    let mut cargo_flags: Vec<String> = vec![format!("--features={}", FEATURES.join(","))];
    if profile == "release" {
        cargo_flags.push("--release".to_string());
    }

    check_command("cargo", "cargo not found. Install Rust");
    check_cargo_ndk();

    println!("\nBuilding non strip version(profile: {profile})...");
    let strip_config = ["--config", "profile.release.strip=false"];
    let mut cmd = Command::new("cargo");
    cmd.arg("ndk")
        .arg("-t")
        .arg(TARGETS[0]) // Just build for the first target to get the unstripped libs
        .args(vec!["-o", jni_libs_dir.to_str().unwrap()])
        .arg("build")
        .args(&cargo_flags)
        .args(strip_config)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd.status()?;
    if !status.success() {
        return Err("No strip build failed".to_string().into());
    }

    println!("\nGenerating Kotlin bindings...");
    fs::create_dir_all(&bindings_dir)?;

    let reference_lib = jni_libs_dir.join(TARGETS[0]).join(format!("{LIB_NAME}.so"));
    let uniffi_config = script_dir.join("uniffi-android.toml");

    // Execute bindgen
    let mut bindgen_cmd = Command::new("cargo");
    bindgen_cmd
        .arg("run")
        .args(&cargo_flags)
        .arg("--bin")
        .arg(BINDGEN_BIN)
        .arg("generate")
        .arg("--config")
        .arg(&uniffi_config)
        .arg("--library")
        .arg(&reference_lib)
        .arg("--language")
        .arg("kotlin")
        .arg("--out-dir")
        .arg(&bindings_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    // Print out the command used
    println!(
        "({})",
        bindgen_cmd
            .get_args()
            .map(|s| s.to_str().unwrap().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let status = bindgen_cmd.status()?;
    if !status.success() {
        return Err("Binding generation failed".into());
    }

    println!("  ✔ Bindings written to bindgen/");

    println!("Building {CRATE_NAME} for Android targets (profile: {profile})...");

    let platform_args = TARGETS
        .iter()
        .flat_map(|target| ["-t", target])
        .collect::<Vec<_>>();

    let mut cmd = Command::new("cargo");
    cmd.arg("ndk")
        .args(&platform_args)
        .args(vec!["-o", jni_libs_dir.to_str().unwrap()])
        .arg("build")
        .args(&cargo_flags)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd.status()?;
    if !status.success() {
        return Err("Build failed".to_string().into());
    }

    println!("\nDone!");
    Ok(())
}

/// Simple check to ensure a command exists in the user's PATH
fn check_command(cmd: &str, error_msg: &str) {
    if Command::new(cmd).arg("--version").output().is_err() {
        eprintln!("❌ {}", error_msg);
        std::process::exit(1);
    }
}

/// Specifically check for cargo-ndk
fn check_cargo_ndk() {
    let output = Command::new("cargo").arg("ndk").arg("--version").output();
    if output.is_err() || !output.unwrap().status.success() {
        eprintln!("❌ cargo-ndk not found. Install it with: cargo install cargo-ndk");
        std::process::exit(1);
    }
}
