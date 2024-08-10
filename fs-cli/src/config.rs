//! Manage configs from file, command-line arguments, and default values.
//! Config priority: Command-line arguments > Config file > Default value

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to the config file.
    #[clap(short, long, value_parser, value_name = "FILE")]
    config: Option<PathBuf>,

    #[clap(flatten)]
    args: CliArgs,
}

#[derive(Debug, Serialize, Deserialize, clap::Args)]
struct CliArgs {
    /// The path to the directory or file to be analyzed.
    #[clap(short, long, value_parser, value_name = "PATH")]
    path: Option<PathBuf>,
}

/// 引数、ファイルからの設定をよしなに管理する構造体
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// 実行するディレクトリ、またはファイルのパス
    pub path: PathBuf,
}

impl Config {
    /// テスト時にコマンドライン引数をシミュレートできるように、Cliのインスタンスを渡す
    pub fn new(cli: Cli) -> Self {
        let default_config = Config::default();
        let file_config = cli
            .config
            .as_ref()
            .map(Config::read_config_from_file)
            .transpose()
            .unwrap();

        Config::merge_configs(default_config, file_config, cli.args)
    }

    /// 指定された設定ファイルから設定を読みこむ
    fn read_config_from_file(file_path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    fn merge_configs(default: Config, file: Option<Config>, cli: CliArgs) -> Config {
        let mut config = serde_json::to_value(default).unwrap();
        if let Some(file_config) = file {
            let file_value = serde_json::to_value(file_config).unwrap();
            Config::json_merge(&mut config, &file_value);
        }
        let cli_value = serde_json::to_value(cli).unwrap();
        Config::json_merge(&mut config, &cli_value);
        serde_json::from_value(config).unwrap()
    }

    fn json_merge(a: &mut serde_json::Value, b: &serde_json::Value) {
        match (a, b) {
            (a @ &mut serde_json::Value::Object(_), serde_json::Value::Object(b)) => {
                let a = a.as_object_mut().unwrap();
                for (k, v) in b {
                    Config::json_merge(a.entry(k.clone()).or_insert(serde_json::Value::Null), v);
                }
            }
            (a, b) => {
                if !b.is_null() {
                    *a = b.clone();
                }
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: PathBuf::from("./"),
        }
    }
}

// TODO: Add tests.
