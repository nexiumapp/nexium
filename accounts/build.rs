/// Compile the gRPC protocol files.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./accounts.proto")?;
    Ok(())
}
