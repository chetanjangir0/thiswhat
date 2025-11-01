use std::{env, process};
use thiswhat::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("problem parsing arguments: {}", e);
        process::exit(1);
    });

    println!("{:?}", config.path)
}
