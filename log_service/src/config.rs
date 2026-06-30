use crate::level::LogLevel;

pub struct LoggerConfig {
    pub level: LogLevel,
    pub console: bool,
    pub file: Option<String>,
}