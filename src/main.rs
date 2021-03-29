use indoc::formatdoc;
use std::fmt;
use std::fs;
use std::process::Command;
use std::{env, path::PathBuf};
use structopt::StructOpt;

extern crate dirs;

#[derive(Debug, StructOpt)]
#[structopt(name = "autod")]
struct Opt {
    command: PathBuf,

    #[structopt(subcommand)]
    sub_command: Option<When>,
}

#[derive(Debug, StructOpt)]
enum When {
    When { time: String },
}

#[derive(Debug)]
enum Target {
    SimpleTarget(String),
    No,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn get_target(args: Vec<String>) -> Result<Target, String> {
    if args.len() < 3 {
        return Ok(Target::No);
    }
    let when = &args[2];
    if when == "when" {
        let target = &args[3];
        Ok(Target::SimpleTarget(target.into()))
    } else {
        Ok(Target::No)
    }
}

fn main() {
    let opt = Opt::from_args();
    let args: Vec<String> = env::args().collect();
    let prog = &args[1].clone();

    let target = get_target(args).expect("Failed to parse when parameter");

    let output = Command::new("sh")
        .arg("-c")
        .arg(String::from("command -v ") + prog)
        .output()
        .expect("failed to execute command -v");
    let progpath = String::from_utf8(output.stdout).unwrap();

    // let configPath = env::var("XDG_CONFIG_HOME");
    let mut service_file = dirs::config_dir().unwrap();
    service_file.push("systemd/user");
    service_file.push(prog);
    service_file.set_extension("service");

    let content = formatdoc!(
        "
        [Unit]
        Description=Runs {}, created by autod

        [Service]
        ExecStart={}
        {}
        ",
        prog,
        progpath,
        target
    );
    fs::write(service_file, content).expect("Unable to write Service File");
}
