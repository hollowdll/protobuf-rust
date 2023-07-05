# Overview

Protocol Buffers Rust implementation example.

This example uses prost and prost-build Rust libraries for protobuf runtime and code generation from .proto files.

This also demonstrates the performance differences between Protocol Buffers and JSON. See `main.rs` in `src`.

Results show that Protocol Buffers is much faster than JSON when it comes to large amounts of data.

# Protocol Buffers

Protocol Buffers is a blazingly fast data format for serialized structured data. Unlike JSON, Protocol Buffers can be encoded to binary format, making it very efficient and fast.

# Build

No configurations. `build.rs` generates Rust files from .proto files at compile time.

```bash
git clone
```

```bash
cd protobuf-rust
```

```bash
cargo run
```

# Usage

Creates a given number of employee instances both in Protocol Buffers format and JSON format.

Writes them to files and reads them. After this the program shows how long it took for each to complete.

```bash
cargo run -- --help
```

To change the number of created employees for better demonstration.

```bash
cargo run -- -e <NUMBER OF EMPLOYEES>
```

NOTE! Try to change the number to over 10000 to see better results.
