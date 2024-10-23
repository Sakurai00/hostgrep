use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(help = "検索ワード")]
    search_word: String,
}

fn main() -> Result<()> {
    let path: &str = "/etc/hosts";
    let args = Args::parse();

    hostgrep(path, args.search_word)?;

    Ok(())
}

fn hostgrep(path: &str, word: String) -> Result<()> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    for line in buf.lines() {
        let line = line?;
        if line.contains(&word) && !line.trim().starts_with('#') {
            println!("{}", line);
        }
    }
    Ok(())
}
