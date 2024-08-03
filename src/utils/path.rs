use std::path::MAIN_SEPARATOR;

/// 実行環境に合わせたパスの区切り文字に変換するi
pub fn convert_to_native_path(path: &str) -> String {
    path.replace("/", &String::from(MAIN_SEPARATOR))
}
