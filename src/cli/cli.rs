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
        default_value = "false",
        long_help = "Print debug information"
    )]
    pub debug: bool,
}
