#![allow(unused)]

use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Read, StdoutLock, Write},
};

use anyhow::{Context, Result};
use clap::Parser;
use log::info;

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
    env_logger::init();
    info!("starting up!");

    let args = Cli::parse();
    info!("parsing params");

    info!("starting opening file");
    let file = File::open(&args.filepath)
        .with_context(|| format!("could not read file: `{:?}`", &args.filepath))?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader.read_to_string(&mut contents);

    info!("getting lock to stdout");
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    find_matches(&contents, &args.pattern, &mut handle);

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut handle: impl Write) {
    for line in content.lines() {
        info!("reading line {:?}", line);
        if line.contains(pattern) {
            writeln!(handle, "{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n")
}
