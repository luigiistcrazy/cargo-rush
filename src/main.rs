use crossterm::style::Stylize;

mod cli;
mod commands;
mod macros;
mod toml;

fn main() {
    #[cfg(debug_assertions)]
    debug!("Running in debug mode");
    cli::main();
}
