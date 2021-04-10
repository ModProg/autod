#![feature(non_ascii_idents)]
#![allow(uncommon_codepoints, dead_code)]
use clap::Clap;
use dirs;
use indoc::{formatdoc, printdoc};
use std::process::Command;
use std::{env, fs};
use std::{path::PathBuf, str};
use unwrap::unwrap;

mod cli;
use cli::{Opt, Target};

// TODO This should be solved better
use crate::cli::calendar::TimerAble;

// FIXME Is this the right file to hold this?
impl Target {
    fn timer(&self) -> Option<String> {
        match self {
            Target::When { timer } => Some(timer.timer()),
            _ => None,
        }
    }

    fn install(&self) -> Option<String> {
        match self {
            Target::On { event } => {
                Some(String::from("WantedBy=") + event)
            }
            _ => None,
        }
    }
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
    let prog = opt.command;
    let (progpath, progname) = match 1 {
        _ if prog.is_absolute() => (prog, None),
        _ if prog
            .parent()
            .map_or(true, |p| p.as_os_str().is_empty()) =>
        {
            if prog.is_file() {
                eprintln!(
                    "There is a local file {0}, if you want to use that \
                    use ./{0}",
                    prog.display()
                )
            }
            let progname = unwrap!(
                prog.file_name(),
                "{} is not a valid Programname.",
                prog.display()
            )
            .to_string_lossy();
            let output = Command::new("sh")
                .arg("-c")
                .arg(String::from("command -v ") + &progname)
                .output()
                .expect("failed to execute command -v");

            (
                PathBuf::from(
                    str::from_utf8(output.stdout.as_slice())
                        .expect(
                            "The return of command intrestingly was invalid \
                            Unicode.",
                        )
                        .trim(),
                ),
                Some(progname.as_ref().to_owned()),
            )
        }
        _ => {
            let mut abs_path = unwrap!(
                env::current_dir(),
                "Unable to resolve relative path {} due to invalid working \
                directory. Make sure autod has sufficient permissions or use \
                an absolute path",
                prog.display()
            );

            abs_path.push(prog);

            (abs_path, None)
        }
    };

    let target = opt.target.unwrap_or_default();
    let custom_output_dir = opt.output_dir.is_some();
    let mut service_file = opt.output_dir.unwrap_or_else(|| {
        let mut file = dirs::config_dir()
            .expect("Could not find the config Directory");
        file.push("systemd/user");
        file
    });
    if !service_file.is_dir() {
        fs::create_dir_all(&service_file).expect(
            "Could not find or create the systemd config folder.",
        );
        if !custom_output_dir {
            println!(
                "Created {}, make sure you have systemd installed.",
                service_file.display()
            );
        }
    }

    let service_name = sanitize_filename::sanitize_with_options(
        opt.name.unwrap_or_else(|| {
            progname.unwrap_or_else(|| {
                progpath.to_string_lossy().as_ref().to_owned()
            })
        }),
        sanitize_filename::Options {
            windows: opt.windows,
            truncate: false,
            replacement: "�",
        },
    );

    service_file.push(&service_name);
    service_file.set_extension("service");

    let timer_file = service_file.with_extension("timer");

    if !opt.print {
        match (&target, opt.overwrite) {
            (_, false) if service_file.exists() => {
                panic!(
                "The service file {} already exists, consider using -o to \
                overwrite, -p to print, or -n to provide a different name",
                service_file.display()
            )
            }
            (_, true) if service_file.is_dir() => {
                panic!(
                "The service file {} is a directory, consider moving it or \
                using -p to print or -n to provide a different name",
                service_file.display()
            )
            }
            (Target::When { timer: _ }, false)
                if timer_file.exists() =>
            {
                panic!(
                "The timer file {} already exists, consider using -o to \
                overwrite, -p to print, or -n to provide a different name",
                timer_file.display()
            )
            }
            (Target::When { timer: _ }, true)
                if timer_file.is_dir() =>
            {
                panic!(
                "The timer file {} is a directory, consider moving it or \
                using -p to print or -n to provide a different name",
                timer_file.display()
            )
            }
            _ => {}
        }
    }

    let mut service = formatdoc!(
        "[Unit]
        Description=Runs {}, created by autod

        [Service]
        ExecStart={}
        ",
        service_name,
        progpath.to_str().unwrap()
    );

    if let Some(install) = target.install() {
        service = formatdoc!(
            "{}[Install]
            {}
            ",
            service,
            install
        )
    }

    if opt.print {
        if let Target::When { timer: _ } = target {
            printdoc!(
                "
                Service File:
                ===
                {}
                ===

                ",
                service
            );
        } else {
            println!("{}", service);
        }
    } else {
        fs::write(&service_file, service)
            .expect("Unable to write Service File");
    }

    if let Some(timer) = target.timer() {
        let timer_content = formatdoc!(
            "
        [Unit]
        Description=Runs {} on a timer
        
        [Timer]
        {}

        [Install]
        WantedBy=timers.target
        ",
            service_name,
            timer
        );
        if opt.print {
            printdoc!(
                "
                Timer File:
                ===
                {}
                ===
                ",
                timer_content
            );
        } else {
            fs::write(&timer_file, timer_content)
                .expect("Unable to write Timer File");
        }
    }
}
