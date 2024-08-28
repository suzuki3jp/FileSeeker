// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::path::PathBuf;

use fs_core::directory_scanner::DirectoryScanner;
use fs_core::file_analyzer::{FileAnalysisResult, FileAnalyzer};
use serde::Serialize;

#[derive(Serialize)]
struct AnalysisResult {
    path_parts: Vec<String>,
    extension: String,
    line: isize,
    char: isize,
    size: u64,
}

/// ファイルを解析する関数
/// ファイルの内容がutf-8ではなかった場合行数や文字数はマイナスを返す
#[tauri::command]
fn analyze(path: &str) -> Result<Vec<AnalysisResult>, String> {
    let p = PathBuf::from(path);
    let scanner = DirectoryScanner::new();

    let mut analysis_results: Vec<AnalysisResult> = vec![];

    let file_paths = scanner.scan(&p).map_err(|e| e.to_string())?;

    for f in file_paths {
        let analyzer = FileAnalyzer::new(&f);
        let analyzed_file_data = analyzer.analyze().map_err(|e| e.to_string())?;

        let FileAnalysisResult {
            path_parts,
            extension,
            line,
            char,
            size,
        } = analyzed_file_data;

        let analysis_result = AnalysisResult {
            path_parts,
            extension,
            line: line.map_or(-1, |v| v as isize),
            char: char.map_or(-1, |v| v as isize),
            size,
        };

        analysis_results.push(analysis_result);
    }

    Ok(analysis_results)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![analyze])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
