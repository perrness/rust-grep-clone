#![allow(unused)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let file = File::open(args.filepath)?;
    let mut reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.expect("Can't read line from file :S");
        if l.contains(&args.pattern) {
            println!("{}", l);
        }
    }

    Ok(())
}
