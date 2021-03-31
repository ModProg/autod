mod cli;

use clap::Clap;
use dirs;
use indoc::formatdoc;
use std::fs;
use std::process::Command;
use std::{fmt, path::PathBuf, str};

use cli::{Opt, Target};

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::When { time } => write!(f, "{:?}", time),
            Target::On { event } => write!(f, "[Install]\nWantedBy={}", event),
            Target::No => write!(f, ""),
        }
    }
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
    let prog = opt.command;
    let progpath = if prog.is_absolute() {
        prog
    } else {
        let progname = prog.file_name().map(|p| p.to_str()).unwrap().unwrap();
        let output = Command::new("sh")
            .arg("-c")
            .arg(String::from("command -v ") + progname)
            .output()
            .expect("failed to execute command -v");
        PathBuf::from(str::from_utf8(output.stdout.as_slice()).unwrap().trim()) //.expect("Unable to finde Program ")
    };

    let progname = progpath.file_name().unwrap().to_str().unwrap();

    let target = opt.target.unwrap_or_default();

    // let configPath = env::var("XDG_CONFIG_HOME");
    let mut service_file = dirs::config_dir().unwrap();
    service_file.push("systemd/user");
    service_file.push(progname);
    service_file.set_extension("service");

    let content = formatdoc!(
        "
        [Unit]
        Description=Runs {}, created by autod

        [Service]
        ExecStart={}

        {}
        ",
        progname,
        progpath.to_str().unwrap(),
        target
    );
    fs::write(service_file, content).expect("Unable to write Service File");
}
