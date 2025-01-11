use std::path;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: path::PathBuf
}

/// executed by cargo run --bin <pattern> <filename>
fn main() {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("cound not read file");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
