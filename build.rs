// Build script to build .proto files into Rust files during Cargo build.

use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["src/employees.proto"], &["src/"])?;
    Ok(())
}
