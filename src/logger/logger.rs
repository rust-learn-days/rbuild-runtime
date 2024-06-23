use std::io::Write;

use colored::Colorize;
use env_logger::Builder;
use log::{Level, LevelFilter};

pub fn init_logger(filter: LevelFilter) {
    Builder::new()
        .format(|buf, record| {
            let level_color = match record.level() {
                Level::Error => "ERROR".red(),
                Level::Warn => "WARN".yellow(),
                Level::Info => "INFO".green(),
                Level::Debug => "DEBUG".blue(),
                Level::Trace => "TRACE".white(),
            };
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
                    .cyan(),
                level_color,
                record.args()
            )
        })
        .filter(None, filter)
        .init();
}

#[cfg(test)]
mod tests {
    use log::{debug, error, info, trace, warn};

    use super::*;

    #[test]
    fn test_logging() {
        init_logger(LevelFilter::Trace);
        info!("{}", "This is another info message with color");
        error!("{}", "This is error msg");
        warn!("{}", "This is warn msg");
        debug!("{}", "This is debug msg");
        trace!("{}", "This is trace msg");
    }
}
