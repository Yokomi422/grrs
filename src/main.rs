use std::process;
use std::path;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: path::PathBuf
}

fn main() {
    let args = Cli::parse();
}
