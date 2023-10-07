fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = vec![
        "protos/hello.proto",
        "protos/account.proto",
        "protos/profile.proto",
        "protos/tweet.proto",
    ];

    tonic_build::configure()
        .build_server(true)
        .compile(&proto_files, &["protos/"])?;

    Ok(())
}
