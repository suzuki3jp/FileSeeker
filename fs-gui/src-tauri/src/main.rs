// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::path::PathBuf;

use fs_core::directory_scanner::DirectoryScanner;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn analyze(path: &str) {
    let p = PathBuf::from(path);
    let scanner = DirectoryScanner::new();

    match scanner.scan(&p) {
        Ok(files) => {
            println!("Found files:\n");
            for f in files {
                println!("{}", f.to_string_lossy());
            }
        }
        Err(e) => println!("Error analyzing files: {}", e),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, analyze])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
