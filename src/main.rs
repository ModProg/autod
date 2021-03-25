use std::env;
use std::fs;
use std::process::Command;

extern crate dirs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = &args[1];

    let output = Command::new("sh")
        .arg("-c")
        .arg(String::from("command -v ") + prog)
        .output()
        .expect("failed to execute command -v");
    let progpath = String::from_utf8(output.stdout).unwrap();

    // let configPath = env::var("XDG_CONFIG_HOME");
    let home = dirs::home_dir().unwrap();
    fs::write(
        home + String::from(".config/systemd/user/") + prog,
        progpath,
    )
    .expect("Unable to write Service File");
}
