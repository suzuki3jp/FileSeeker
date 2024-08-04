mod config;
mod directory_scanner;

use clap::Parser;
use config::{Cli, Config};
use directory_scanner::DirectoryScanner;

fn main() {
    let config = Config::new(Cli::parse());

    let scanner = DirectoryScanner::new();

    match scanner.scan(&config.path) {
        Ok(files) => {
            println!("Found files:\n ");
            for f in files {
                println!("{}", f.to_string_lossy())
            }
        }
        Err(e) => println!("Error scanning dir: {}", e),
    }
}
