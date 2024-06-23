use std::fmt;
use std::str::FromStr;

use clap::Parser;

use crate::BuildRuntime;

#[derive(Parser, Debug)]
#[command(name = "rbuild-runtime", about, version, author, long_about = None)]
pub struct BuildRuntimeOpts {
    #[clap(subcommand)]
    pub cmd: BuildRuntime,
    #[arg(
        short,
        long,
        default_value = "info",
        long_help = "Set the log level for the application. Supported log levels are error, info, warn, debug, trace.",
        value_parser = parse_log_level,
    )]
    pub log_level: LogLevel,
}

#[derive(Debug, Copy, Clone)]
pub enum LogLevel {
    ERROR,
    INFO,
    WARN,
    DEBUG,
    TRACE,
}

impl From<LogLevel> for &'static str {
    fn from(f: LogLevel) -> Self {
        match f {
            LogLevel::ERROR => "error",
            LogLevel::INFO => "info",
            LogLevel::WARN => "warn",
            LogLevel::DEBUG => "debug",
            LogLevel::TRACE => "trace",
        }
    }
}

fn parse_log_level(s: &str) -> Result<LogLevel, &'static str> {
    s.parse()
}

impl FromStr for LogLevel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "error" => Ok(LogLevel::ERROR),
            "info" => Ok(LogLevel::INFO),
            "warn" => Ok(LogLevel::WARN),
            "debug" => Ok(LogLevel::DEBUG),
            "trace" => Ok(LogLevel::TRACE),
            _ => Err("Invalid Log Level"),
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
