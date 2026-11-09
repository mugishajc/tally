//! rwc: a parallel word count CLI.

use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use rayon::prelude::*;

use rwc::{count, Counts};

#[derive(Parser)]
#[command(name = "rwc", version, about = "Parallel word count in Rust")]
struct Cli {
    /// Count lines
    #[arg(short = 'l', long)]
    lines: bool,
    /// Count words
    #[arg(short = 'w', long)]
    words: bool,
    /// Count bytes
    #[arg(short = 'c', long)]
    bytes: bool,
    /// Count characters
    #[arg(short = 'm', long)]
    chars: bool,
    /// Files to count. If empty, reads stdin.
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    // Default to lines + words + bytes like GNU wc with no flags.
    let (l, w, b, m) = if !(cli.lines || cli.words || cli.bytes || cli.chars) {
        (true, true, true, false)
    } else {
        (cli.lines, cli.words, cli.bytes, cli.chars)
    };

    if cli.files.is_empty() {
        let c = count(io::stdin().lock()).context("read stdin")?;
        println!("{}", format_line(&c, l, w, b, m, None));
        return Ok(());
    }

    let results: Vec<(PathBuf, Result<Counts>)> = cli
        .files
        .par_iter()
        .map(|p| (p.clone(), count_file(p)))
        .collect();

    let mut total = Counts::default();
    let mut any_err = false;
    for (path, res) in &results {
        match res {
            Ok(c) => {
                total.add(c);
                println!("{}", format_line(c, l, w, b, m, Some(path)));
            }
            Err(e) => {
                any_err = true;
                eprintln!("rwc: {}: {}", path.display(), e);
            }
        }
    }
    if results.len() > 1 {
        println!("{}", format_line(&total, l, w, b, m, Some(Path::new("total"))));
    }
    if any_err {
        std::process::exit(1);
    }
    Ok(())
}

fn count_file(p: &Path) -> Result<Counts> {
    let f = File::open(p).with_context(|| format!("open {}", p.display()))?;
    count(f).with_context(|| format!("read {}", p.display()))
}

fn format_line(c: &Counts, l: bool, w: bool, b: bool, m: bool, name: Option<&Path>) -> String {
    let mut out = String::new();
    let mut push = |n: u64| {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(&format!("{:>8}", n));
    };
    if l { push(c.lines); }
    if w { push(c.words); }
    if b { push(c.bytes); }
    if m { push(c.chars); }
    if let Some(p) = name {
        out.push(' ');
        out.push_str(&p.display().to_string());
    }
    out
}
