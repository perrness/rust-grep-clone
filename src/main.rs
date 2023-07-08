#![allow(unused)]

use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contains it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short, long)]
    pattern: String,

    /// The path to the file to look in
    #[arg(short = 'f', long)]
    filepath: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.filepath)
        .with_context(|| format!("could not read file: `{:?}`", &args.filepath))?;
    let mut reader = BufReader::new(file);

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    for line in reader.lines() {
        let l = line.expect("Can't read line from file :S");
        if l.contains(&args.pattern) {
            writeln!(handle, "{}", l);
        }
    }

    Ok(())
}
