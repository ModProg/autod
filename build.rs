use clap::IntoApp;
use clap_generate::{self, generators};
use std::env;

include!("src/cli.rs");

fn main() {
    let mut outdir = match env::var_os("CARGO_MANIFEST_DIR") {
        None => return,
        Some(outdir) => outdir,
    };

    outdir.push("/target/comp");

    let mut app = Opt::into_app();

    clap_generate::generate_to::<generators::Zsh, _, _>(
        &mut app, "autod", outdir,
    )
}
