//! Counting primitives shared between the CLI and the tests.

use std::io::{BufRead, BufReader, Read};

/// Counts for one input stream.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    pub lines: u64,
    pub words: u64,
    pub bytes: u64,
    pub chars: u64,
}

impl Counts {
    pub fn add(&mut self, other: &Counts) {
        self.lines += other.lines;
        self.words += other.words;
        self.bytes += other.bytes;
        self.chars += other.chars;
    }
}

/// Counts lines, words, bytes and characters while streaming the reader, so
/// files bigger than memory still work.
pub fn count<R: Read>(reader: R) -> std::io::Result<Counts> {
    let mut r = BufReader::new(reader);
    let mut counts = Counts::default();
    let mut buf = Vec::with_capacity(64 * 1024);

    loop {
        buf.clear();
        let n = r.read_until(b'\n', &mut buf)?;
        if n == 0 {
            break;
        }
        counts.bytes += n as u64;
        if buf.ends_with(b"\n") {
            counts.lines += 1;
        }
        // Words: any run of non-whitespace bytes.
        let mut in_word = false;
        for &b in &buf {
            let is_ws = b == b' ' || b == b'\t' || b == b'\n' || b == b'\r';
            if !is_ws && !in_word {
                counts.words += 1;
                in_word = true;
            } else if is_ws {
                in_word = false;
            }
        }
        // Characters: decode as UTF-8, count scalar values. Invalid bytes are
        // counted as one character each (lossy decode), matching wc -m.
        counts.chars += String::from_utf8_lossy(&buf).chars().count() as u64;
    }
    Ok(counts)
}
