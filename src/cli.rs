use clap::{Parser, Subcommand, crate_authors, crate_description, crate_version};
use crossterm::style::Stylize;

use crate::commands;

#[cfg(debug_assertions)]
use crate::debug;

#[derive(Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Jet(JetArgs),
}

#[derive(clap::Args, Debug)]
#[command(
    author = crate_authors!(),
    version = crate_version!(),
    about = crate_description!(),
    long_about = None,
    display_name = "cargo jet"
)]
struct JetArgs {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(about = "Initializes a new project")]
    Init,
}

pub fn main() {
    let CargoCli::Jet(jet_args) = CargoCli::parse();

    #[cfg(debug_assertions)]
    debug!(&format!("CLI parsed: {:?}", jet_args));

    match &jet_args.command {
        Command::Init => commands::init::cli(),
    }
}
