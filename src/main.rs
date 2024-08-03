mod config;
mod directory_scanner;

use clap::Parser;
use config::{Cli, Config};
use directory_scanner::DirectoryScanner;

fn main() {
    let config = Config::new(Cli::parse());

    let scanner = DirectoryScanner::new();

    match scanner.scan(config.path.display().to_string()) {
        Ok(files) => {
            println!("Found files:\n ");
            for f in files {
                println!("{}", f)
            }
        }
        Err(e) => println!("Error scanning dir: {}", e),
    }
}
