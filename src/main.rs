// src/main.rs
/*
 * Main executable for CryptoManager
 */

use clap::Parser;
use cryptomanager::{Result, run};

#[derive(Parser)]
#[command(version, about = "CryptoManager - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
