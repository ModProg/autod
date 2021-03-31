use clap::{AppSettings, Clap};
use std::default::Default;
use std::path::PathBuf;

#[derive(Debug, Clap)]
#[clap(
    name = "autod",
    version = "0.1",
    author = "Roland F. <important@van-fredenhagen.de>",
    bin_name = "autod"
)] //, global_setting = AppSettings::DisableHelpSubcommand)]
pub struct Opt {
    pub command: PathBuf,

    #[clap(subcommand)]
    pub target: Option<Target>,

    #[clap(long, short, global = true)]
    pub print: bool,

    #[clap(long, short, global = true)]
    pub overwrite: bool,
}

#[derive(Debug, Clap)]
pub enum Target {
    When {
        time: String,
    },
    On {
        event: String,
    },
    #[clap(setting(AppSettings::Hidden))]
    No,
}

impl Default for Target {
    fn default() -> Self {
        Target::No
    }
}
