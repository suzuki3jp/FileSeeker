mod directory_scanner;

use clap::Parser;
use directory_scanner::DirectoryScanner;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("./"))]
    path: String,
}

fn main() {
    let args = Args::parse();

    let scanner = DirectoryScanner::new();

    match scanner.scan(args.path) {
        Ok(files) => {
            println!("Found files:\n ");
            for f in files {
                println!("{}", f)
            }
        }
        Err(e) => println!("Error scanning dir: {}", e),
    }
}
