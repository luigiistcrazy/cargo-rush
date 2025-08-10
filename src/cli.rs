use clap::{Parser, crate_authors, crate_description, crate_version};

use crate::commands;

#[derive(Parser, Debug)]
#[clap(author = crate_authors!(), version = crate_version!(), about = crate_description!(), long_about = None, bin_name = "cargo rush")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    #[clap(about = "Initializes a new project")]
    Init,
}

pub fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Command::Init => commands::init::cli(),
    }
}
