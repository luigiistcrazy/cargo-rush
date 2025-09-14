mod cli;
mod commands;
mod macros;

fn main() {
    #[cfg(debug_assertions)]
    debug!("Running in debug mode");
    cli::main();
}
