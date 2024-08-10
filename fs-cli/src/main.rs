mod config;

use clap::Parser;
use config::{Cli, Config};
use fs_core::directory_scanner::DirectoryScanner;
use fs_core::file_analyzer::{FileAnalysisResult, FileAnalyzer};

fn main() {
    let config = Config::new(Cli::parse());

    let scanner = DirectoryScanner::new();

    match scanner.scan(&config.path) {
        Ok(files) => {
            println!("Found files:\n ");
            for f in files {
                let analyzer = FileAnalyzer::new(&f);
                let result = analyzer.analyze();

                match result {
                    Ok(r) => {
                        let FileAnalysisResult {
                            path_parts,
                            extension,
                            line,
                            char,
                            size,
                        } = r;

                        let content = format!(
                            "{}, {}, {}, {}, {}",
                            path_parts.join("/"),
                            extension,
                            line,
                            char,
                            size
                        );
                        println!("{}", content);
                    }
                    Err(e) => println!("Error analyzing file: {}", e),
                }
            }
        }
        Err(e) => println!("Error scanning dir: {}", e),
    }
}
