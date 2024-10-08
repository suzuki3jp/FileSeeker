use std::fs;
use std::io;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;

#[derive(Default)]
pub struct DirectoryScanner {}

impl DirectoryScanner {
    pub fn new() -> Self {
        Self::default()
    }

    /// 指定ディレクトリのすべての階層をスキャンし、ファイルパスを収集する
    pub fn scan(&self, path: &PathBuf) -> io::Result<Vec<PathBuf>> {
        let target = Path::new(&path);

        let mut file_paths = Vec::new();

        if target.is_dir() {
            match self.shallow_scan(path) {
                Ok(result) => {
                    file_paths.extend(result.files);

                    for dir in result.dirs {
                        match self.scan(&dir) {
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
    fn shallow_scan(&self, path: &PathBuf) -> io::Result<ShallowScanResult> {
        let target = Path::new(&path);

        let mut dir_paths = Vec::new();
        let mut file_paths = Vec::new();

        if target.is_dir() {
            for e in fs::read_dir(target)? {
                let e = e?;
                let p = e.path();

                if p.is_file() {
                    if let Some(path_str) = p.to_str() {
                        file_paths.push(PathBuf::from(path_str))
                    }
                } else if p.is_dir() {
                    if let Some(path_str) = p.to_str() {
                        dir_paths.push(PathBuf::from(path_str))
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
        Error::new(io::ErrorKind::InvalidInput, "Target path is not directory.")
    }
}

pub struct ShallowScanResult {
    pub dirs: Vec<PathBuf>,
    pub files: Vec<PathBuf>,
}

#[cfg(test)]
mod directory_scanner_tests {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use std::path::MAIN_SEPARATOR;
    use tempfile::TempDir;

    /// 実行環境に合わせたパスの区切り文字に変換するi
    fn convert_to_native_path(path: &str) -> String {
        path.replace("/", &String::from(MAIN_SEPARATOR))
    }

    use super::DirectoryScanner;

    fn create_test_dir() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        fs::create_dir(base_path.join("dir1")).unwrap();
        fs::create_dir(base_path.join("dir2")).unwrap();
        fs::create_dir(base_path.join("dir1").join("subdir")).unwrap();

        File::create(base_path.join("file1.txt"))
            .unwrap()
            .write_all(b"content")
            .unwrap();
        File::create(base_path.join("dir1").join("file2.txt"))
            .unwrap()
            .write_all(b"content")
            .unwrap();
        File::create(base_path.join("dir2").join("file3.txt"))
            .unwrap()
            .write_all(b"content")
            .unwrap();
        File::create(base_path.join("dir1").join("subdir").join("file4.txt"))
            .unwrap()
            .write_all(b"content")
            .unwrap();

        temp_dir
    }

    #[test]
    fn test_new() {
        let scanner = DirectoryScanner::new();
        assert!(scanner.scan(&PathBuf::from("")).is_err());
    }

    #[test]
    fn test_scan() {
        let temp_dir = create_test_dir();
        let scanner = DirectoryScanner::new();

        let result = scanner.scan(&temp_dir.path().to_path_buf());
        assert!(result.is_ok());

        let files = result.unwrap();
        assert_eq!(files.len(), 4); // 全てのファイルが見つかったか確認

        let expected_files = vec![
            "file1.txt",
            "dir1/file2.txt",
            "dir2/file3.txt",
            "dir1/subdir/file4.txt",
        ];

        // 全ての期待されるファイルが含まれているか確認
        for e in expected_files {
            assert!(files.iter().any(|f| f.ends_with(convert_to_native_path(e))));
        }
    }

    #[test]
    fn test_shallow_scan() {
        let temp_dir = create_test_dir();
        let scanner = DirectoryScanner::new();

        let result = scanner.shallow_scan(&temp_dir.path().to_path_buf());
        assert!(result.is_ok());

        let scan_result = result.unwrap();
        assert_eq!(scan_result.files.len(), 1); // ルートディレクトリに1つのファイル
        assert_eq!(scan_result.dirs.len(), 2); // 2つのサブディレクトリ

        assert!(scan_result.files.iter().any(|f| f.ends_with("file1.txt")));
        assert!(scan_result.dirs.iter().any(|d| d.ends_with("dir1")));
        assert!(scan_result.dirs.iter().any(|d| d.ends_with("dir2")));
    }

    #[test]
    fn test_scan_non_existent_directory() {
        let scanner = DirectoryScanner::new();
        let result = scanner.scan(&PathBuf::from("/path/to/non/existent/directory"));
        assert!(result.is_err());
    }
}
