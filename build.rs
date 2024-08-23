fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["src/proto/agenda/v1/agenda.proto"],
            &["src/proto/agenda/v1"],
        ).unwrap_or_else(
        |e| panic!("Failed to compile proto file: {:?}", e)
    );
    Ok(())
}