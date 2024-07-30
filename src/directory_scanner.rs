use std::fs;
use std::io;
use std::io::Error;
use std::path::Path;

pub struct DirectoryScanner {}

impl DirectoryScanner {
    pub fn new() -> Self {
        DirectoryScanner {}
    }

    /// 指定ディレクトリのすべての階層をスキャンし、ファイルパスを収集する
    pub fn scan(&self, path: String) -> io::Result<Vec<String>> {
        let target = Path::new(&path);

        let mut file_paths = Vec::new();

        if target.is_dir() {
            match self.shallow_scan(path) {
                Ok(result) => {
                    file_paths.extend(result.files);

                    for dir in result.dirs {
                        match self.scan(dir) {
                            Ok(res) => file_paths.extend(res),
                            Err(e) => return Err(e),
                        }
                    }
                }
                Err(e) => return Err(e),
            }
            Ok(file_paths)
        } else {
            Err(self.dir_err())
        }
    }

    /// 指定ディレクトリを**一階層のみ**スキャンし、ディレクトリパスとファイルパスを返却する
    fn shallow_scan(&self, path: String) -> io::Result<ShallowScanResult> {
        let target = Path::new(&path);

        let mut dir_paths = Vec::new();
        let mut file_paths = Vec::new();

        if target.is_dir() {
            for e in fs::read_dir(target)? {
                let e = e?;
                let p = e.path();

                if p.is_file() {
                    if let Some(path_str) = p.to_str() {
                        file_paths.push(path_str.to_string())
                    }
                } else if p.is_dir() {
                    if let Some(path_str) = p.to_str() {
                        dir_paths.push(path_str.to_string())
                    }
                }
            }

            Ok(ShallowScanResult {
                dirs: dir_paths,
                files: file_paths,
            })
        } else {
            Err(self.dir_err())
        }
    }

    fn dir_err(&self) -> io::Error {
        return Error::new(io::ErrorKind::InvalidInput, "Target path is not directory.");
    }
}

pub struct ShallowScanResult {
    pub dirs: Vec<String>,
    pub files: Vec<String>,
}
