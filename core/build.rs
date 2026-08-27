fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc_path =
        protoc_bin_vendored::protoc_bin_path().expect("Failed to find vendored protoc");
    unsafe {
        std::env::set_var("PROTOC", protoc_path);
    }

    tonic_prost_build::compile_protos("proto/warpinator.proto")?;
    let out_dir = std::env::var("OUT_DIR")?;
    std::fs::rename(format!("{}/_.rs", out_dir), format!("{}/warpinator.rs", out_dir))?;
    Ok(())
}
