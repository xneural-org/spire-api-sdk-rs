use std::path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const PROTO_ROOT: &str = "vendor/spire-api-sdk/proto";
    const PROTO_GLOB: &str = const_format::formatcp!("{PROTO_ROOT}/**/*.proto");

    let mut proto_files = Vec::new();

    for entry in glob::glob(PROTO_GLOB)? {
        match entry {
            Ok(path) => {
                // Tell Cargo to re-run the build script if this proto file changes
                println!("cargo:rerun-if-changed={}", path.display());
                proto_files.push(path);
            }
            Err(e) => eprintln!("- Encountered glob error: {e}"),
        }
    }

    if proto_files.is_empty() {
        return Err("No .proto files found matching the glob pattern!".into());
    }

    // Compile using tonic-prost-build
    // First argument: slice of proto file paths
    // Second argument: include directories (e.g., root "proto" directory)
    tonic_prost_build::configure()
        .compile_protos(&proto_files, &[path::PathBuf::from(PROTO_ROOT)])?;
    Ok(())
}
