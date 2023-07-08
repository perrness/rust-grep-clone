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
        match line {
            Ok(line) => {
                if line.contains(&args.pattern) {
                    println!("{}", line)
                }
            }
            Err(error) => panic!("Probleming reading line from file!"),
        }
    }

    Ok(())
}
