extern crate glob;

use glob::glob;
use std::error::Error;
use tonic_build;

fn main() -> Result<(), Box<dyn Error>> {
    let proto_files: Vec<String> = glob("protos/**/*.proto")?
        .filter_map(Result::ok)
        .map(|path| path.display().to_string())
        .collect();

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&proto_files, &["protos/"])?;

    Ok(())
}
