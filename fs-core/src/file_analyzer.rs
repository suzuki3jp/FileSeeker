//! パスを受け取ってファイルを解析するモジュール
//!
//! ## 取得するもの
//! - ファイル名の文字数
//! - 拡張子
//! - ファイル行数
//! - ファイルサイズ
//! - ファイル文字数
//!
//! ## 受け取るオプション
//! - ファイル内部まで解析するかどうか

use std::fs::{metadata as read_metadata, File};
use std::io::{BufRead, BufReader, Result};
use std::path::{Path, PathBuf};

pub struct FileAnalyzer {
    path: PathBuf,
}

impl FileAnalyzer {
    pub fn new(path: &Path) -> Self {
        FileAnalyzer {
            path: path.to_path_buf(),
        }
    }

    pub fn analyze(&self) -> Result<FileAnalysisResult> {
        let path_parts = self.get_file_path_parts();
        let extension = self.get_file_extension().unwrap_or("none").to_string();
        let size = self.get_file_size()?;

        // ファイルの内容がutf-8ではなかった場合などでエラー時はNone
        let result = self.get_file_line_and_char();
        let mut line = None;
        let mut char = None;
        if let Ok(r) = result {
            let (l, c) = r;
            line = Some(l);
            char = Some(c);
        }

        Ok(FileAnalysisResult {
            path_parts,
            extension,
            line,
            char,
            size,
        })
    }

    fn get_file_path_parts(&self) -> Vec<String> {
        self.path
            .components()
            .filter_map(|c| c.as_os_str().to_str().map(String::from))
            .collect()
    }

    fn get_file_extension(&self) -> Option<&str> {
        self.path.extension().and_then(|e| e.to_str())
    }

    /// ファイルサイズを取得する（バイト単位）
    fn get_file_size(&self) -> Result<u64> {
        let meta = read_metadata(self.path.clone())?;
        Ok(meta.len())
    }

    fn get_file_line_and_char(&self) -> Result<(usize, usize)> {
        let file = File::open(self.path.clone())?;
        let reader = BufReader::new(file);

        // ファイルの行数はvscodeなどのテキストエディタで確認したときより1少なくカウントされているようなので+1する
        let mut line = 1;
        let mut char = 0;

        for l in reader.lines() {
            let l = l?;
            line += 1;
            char += l.chars().count();
        }

        Ok((line, char))
    }
}

pub struct FileAnalysisResult {
    /// "./path/to/my/file.txt" -> [".", "path", "to", "my", "file.txt"]
    pub path_parts: Vec<String>,

    /// 拡張子
    pub extension: String,

    /// ファイルの行数 ファイルの内容がutf-8ではなかった場合などでエラー時はNone
    pub line: Option<usize>,

    /// ファイルの文字数 ファイルの内容がutf-8ではなかった場合などでエラー時はNone
    pub char: Option<usize>,

    /// ファイルサイズ（バイト単位）
    pub size: u64,
}

// TODO: Add tests
