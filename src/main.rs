use clap::Parser;
use log::{info, trace, LevelFilter};
use rust_embed::Embed;

use rbuild_runtime::{init_logger, BuildRuntimeOpts, LogLevel};

#[derive(Embed)]
#[folder = "files/"]
struct Asset;

fn main() {
    let asset = Asset::get("sealos").unwrap();
    let opts: BuildRuntimeOpts = BuildRuntimeOpts::parse();
    let level = match opts.log_level {
        LogLevel::ERROR => LevelFilter::Error,
        LogLevel::INFO => LevelFilter::Info,
        LogLevel::WARN => LevelFilter::Warn,
        LogLevel::DEBUG => LevelFilter::Debug,
        LogLevel::TRACE => LevelFilter::Trace,
    };
    init_logger(level);
    info!("log level set to: {:?}", opts.log_level);
    trace!("asset: {:?}", asset.data.len());
}
