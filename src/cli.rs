use clap::{AppSettings, Clap};
use std::default::Default;
use std::path::PathBuf;

#[path = "calendar.rs"]
pub mod calendar;
use calendar::Timer;

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

    /// Print output to Terminal
    ///
    /// Instead of creating service/timer files, autod will print the files to
    /// stdout.
    #[clap(long, short, global = true)]
    pub print: bool,

    /// When set, a service/timer file with the same name will be overwritten
    #[clap(long, short, global = true)]
    pub overwrite: bool,

    /// Creates a oneshot service
    ///
    /// This means, when starting the service, while it is still running,
    /// a second instance will be created
    #[clap(long, short = 's', global = true)]
    pub oneshot: bool,

    /// Name of the created Service
    ///
    /// Will be used for the filename and description
    #[clap(long, short, global = true)]
    pub name: Option<String>,

    /// Directory for the service files
    ///
    /// Defaults to $XDG_CONFIG_HOME/systemd/user
    #[clap(long, short = 'c', global = true)]
    pub output_dir: Option<PathBuf>,

    /// Sanitizes Filenames for Windows
    ///
    /// Use this if you are storing your service files on a windows partition
    #[clap(long, short, global = true)]
    pub windows: bool,
}

#[derive(Debug, Clap)]
pub enum Target {
    When {
        timer: Timer,
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
