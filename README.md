# rustgrep

A fast, grep-like search tool written in Rust for learning purposes. Supports literal, case-insensitive, and full regex matching with parallel file search.

## Features

- Literal, case-insensitive, and regex search
- Parallel search across multiple files using [rayon](https://github.com/rayon-rs/rayon)
- Recursive directory traversal
- Colored output (filename in cyan, line numbers in green)
- Invert match (`-v`)
- Count-only mode (`-c`)
- Standard grep exit codes

## Installation

**Prerequisites:** Rust toolchain ([install via rustup](https://rustup.rs))

```bash
git clone https://github.com/your-username/rustgrep.git
cd rustgrep
cargo build --release
```

The binary will be at `./target/release/rustgrep`. Optionally install it to your PATH:

```bash
cargo install --path .
```

## Usage

```
rustgrep [OPTIONS] <PATTERN> [PATHS]...
```

### Examples

```bash
# Basic search
rustgrep "TODO" src/main.rs

# Case-insensitive
rustgrep -i "error" logs/app.log

# Regex search
rustgrep -E "^fn [a-z]+" src/lib.rs

# Recursive directory search
rustgrep -r "unwrap()" src/

# Show line numbers
rustgrep -n "panic" src/main.rs

# Invert match — lines that do NOT contain the pattern
rustgrep -v "//.*" src/main.rs

# Count matching lines only
rustgrep -c "warn" logs/app.log

# Combine flags
rustgrep -inr "todo" src/
```

### Flags

| Flag | Long form | Description |
|------|-----------|-------------|
| `-i` | `--ignore-case` | Case-insensitive match |
| `-r` | `--recursive` | Search directories recursively |
| `-n` | `--line-numbers` | Show line numbers |
| `-c` | `--count` | Print count of matches only |
| `-v` | `--invert` | Show lines that do NOT match |
| `-E` | `--regex` | Use full regex matching |

## Project Structure

```
src/
├── main.rs       — entry point, wires all modules together
├── cli.rs        — CLI argument parsing (clap)
├── error.rs      — AppError enum (thiserror)
├── matcher.rs    — Matcher trait + three strategies (literal, case-insensitive, regex)
├── searcher.rs   — file reading and line matching logic (rayon parallel search)
├── walker.rs     — path expansion and directory traversal (walkdir)
└── output.rs     — colored output formatting (owo-colors)
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| [clap](https://crates.io/crates/clap) | CLI argument parsing |
| [regex](https://crates.io/crates/regex) | Regex engine |
| [walkdir](https://crates.io/crates/walkdir) | Recursive directory traversal |
| [rayon](https://crates.io/crates/rayon) | Data parallelism for multi-file search |
| [owo-colors](https://crates.io/crates/owo-colors) | Terminal colors |
| [thiserror](https://crates.io/crates/thiserror) | Ergonomic error types |

## Running Tests

```bash
cargo test
```

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for how to get started.

## License

MIT
