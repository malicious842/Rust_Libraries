use std::fs::OpenOptions;
use std::io::Write;

use crate::config::LoggerConfig;
use crate::level::LogLevel;

pub struct Logger {
    config: LoggerConfig,
}

impl Logger {

    //自分自身をstructに入れている
    pub fn new(config: LoggerConfig) -> Self {
        Self { config }
    }

    pub fn log(&self, level: LogLevel, message: &str) {

        //ログレベルを判定しています
        if level < self.config.level {
            return;
        }

        //ログ文字列を作成しています
        let text = format!("[{:?}] {}", level, message);

        if self.config.console {
            println!("{}", text);
        }

        //if let some はログファイルを作るときに使います
        if let Some(path) = &self.config.file {
            if let Ok(mut file) =
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
            {
                let _ = writeln!(file, "{}", text);
            }
        }
    }

    pub fn debug(&self, msg: &str) {
        self.log(LogLevel::Debug, msg);
    }

    pub fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }

    pub fn warn(&self, msg: &str) {
        self.log(LogLevel::Warn, msg);
    }

    pub fn error(&self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }
}