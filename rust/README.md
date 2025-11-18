# Magika Directory Stats (Rust)

A CLI tool that analyzes files in a directory using [Magika](https://github.com/google/magika) and aggregates capacity by file type.

## Requirements

- Rust 1.91.1+
- Cargo

## Usage

```bash
# Analyze current directory
cargo run --release

# Analyze specific directory
cargo run --release -- /path/to/directory
```

## Output

The tool displays a table showing:
- File type (as detected by Magika)
- Total size per file type
- Percentage of total capacity

## Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

## Development

```bash
# Run linter
cargo clippy -- -D warnings

# Run formatter
cargo fmt

# Run tests
cargo test
```
