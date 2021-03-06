#![feature(non_ascii_idents)]
#![allow(uncommon_codepoints)]
use clap::IntoApp;
use clap_generate::{self, generators};
use std::env;

include!("src/cli.rs");

fn main() {
    let mut outdir = env::var_os("CARGO_MANIFEST_DIR")
        .expect("Unable to find Builddir");
    outdir.push("/target/comp");
    std::fs::create_dir_all(&outdir)
        .expect("Could not create completions dir.");

    let mut app = Opt::into_app();

    clap_generate::generate_to::<generators::Zsh, _, _>(
        &mut app, "autod", outdir,
    );
}
