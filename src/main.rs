use clap::Parser;
use std::path::PathBuf;
mod scan;
mod categorize;
mod grep;

/// Simple program to summerize a project
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path of the project
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    scan::scan(&args.path);
}
