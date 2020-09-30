# rust-lang-ar.github.io
Rust Argentina's GitHub Page

## Development

### Setup

The following crates are required to work with this project

- **wasm-pack**: Compiles Rust to WASM generating a JavaScript interop
- **cargo-watch**: Watch FS events for changes and rebuilds your project
- **simple-http-server**: HTTP Server to serve project assets

```bash
# installing crates using cargo
cargo install wasm-pack
cargo install cargo-watch
cargo install simple-http-server
```

### Source

To run the _development server_ execute the `dev.sh` script available in the `bin/` directory
of the project.

```bash
bash ./bin/dev.sh
```
