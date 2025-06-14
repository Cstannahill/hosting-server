# Custom CLI Tool

This Rust CLI provides handy commands for managing the hosting stack.

## Building

```bash
cd cli_tools/custom_cli_tool
cargo build --release
```

## Example Usage

```bash
./target/release/custom_cli_tool deploy
./target/release/custom_cli_tool logs api
./target/release/custom_cli_tool status
./target/release/custom_cli_tool env get POSTGRES_DB
```
