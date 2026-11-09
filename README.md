# rwc: parallel word count in Rust

A small `wc`-shaped CLI written in Rust. Counts lines, words, bytes and characters. Processes multiple files in parallel with Rayon.

## Install

```bash
cargo install --path .
```

## Use

```bash
rwc README.md
rwc -l -w src/*.rs
echo "hello world" | rwc
```

Flags match `wc`:

- `-l` lines
- `-w` words
- `-c` bytes
- `-m` characters

With no flag it prints lines, words and bytes, like plain `wc`.

## Why

A compact exercise in Rust: streaming I/O on buffered readers, correct UTF-8 character counting without loading the file into memory, concurrent file counts with Rayon's parallel iterators, and a layered CLI vs library split (`lib.rs` is pure counting, `main.rs` is the CLI).

## Tests

```bash
cargo test
```

Covers the counting primitives (empty input, no trailing newline, tabs and spaces, UTF-8 characters vs bytes) and the CLI end to end (stdin, single file, multiple files with a total line, missing file exit code).

CI runs `cargo fmt --check`, `cargo clippy -- -D warnings` and `cargo test --release` on every push.
