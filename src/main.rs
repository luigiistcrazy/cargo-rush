use clap::Parser;
use colored::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Rush(RushArgs),
}

#[derive(clap::Args, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    arg_required_else_help = true
)]
struct RushArgs {
    /// Initialize cargo-rush in current directory
    #[arg(long)]
    init: bool,
}

fn main() {
    print_version_header();

    let args = std::env::args().collect::<Vec<_>>();

    if args.get(1).map(|s| s.as_str()) != Some("rush") {
        show_error("This command must be run as 'cargo rush'");
        std::process::exit(1);
    }

    match CargoCli::parse() {
        CargoCli::Rush(args) => {
            let current_dir = std::env::current_dir().expect("Failed to get current directory");

            check_cargo_toml(&current_dir);
            simulate_loading(150);

            if args.init {
                handle_init(&current_dir);
            } else {
                check_cargorush_file(&current_dir);
                simulate_loading(100);
            }
        }
    }
}

fn print_version_header() {
    println!(
        "\n{} {} {}\n",
        "🚀".cyan(),
        "cargo-rush".bold().cyan(),
        format!("v{}", env!("CARGO_PKG_VERSION")).cyan()
    );
}

fn show_error(message: &str) {
    eprintln!("{} {}", "[✖]".red().bold(), message.red().bold());
}

fn show_warning(message: &str) {
    eprintln!("{} {}", "[⚠]".yellow().bold(), message.yellow().bold());
}

fn show_success(message: &str) {
    println!("{} {}", "[✔]".green().bold(), message.green().bold());
}

fn show_info(message: &str) {
    println!("{} {}", "[i]".blue().bold(), message.blue());
}

fn simulate_loading(millis: u64) {
    // Simulate loading for smoother UX (Can technically be removed if desired)
    print!("{}", "⏳".cyan());
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(millis));
    print!("\r");
}

fn check_cargo_toml(current_dir: &std::path::Path) {
    if !current_dir.join("Cargo.toml").exists() {
        show_warning("Cargo.toml not found in current directory");
        show_info("Make sure you're in the root directory of a Rust project");
        std::process::exit(1);
    }
}

fn check_cargorush_file(current_dir: &std::path::Path) {
    if !current_dir.join(".cargorush.toml").exists() {
        show_warning("cargo-rush not initialized in this project");
        show_info("Run `cargo rush --init` to initialize");
        std::process::exit(1);
    }
}

// Updated handle_init function with visual enhancements
fn handle_init(current_dir: &std::path::Path) {
    let rush_file = current_dir.join(".cargorush.toml");

    if rush_file.exists() {
        show_warning("cargo-rush is already active in this directory");
        show_info("Skipping initialization...");
        return;
    }

    match std::fs::File::create(&rush_file) {
        Ok(_) => {
            show_success("cargo-rush initialized successfully!");
            show_info("It's recommended to add .cargorush.toml to your .gitignore");
        }
        Err(e) => {
            show_error(&format!("Failed to create .cargorush.toml file: {}", e));
            std::process::exit(1);
        }
    }
    populate_cargorush(current_dir);
    handle_gitignore(current_dir);
}

fn handle_gitignore(current_dir: &std::path::Path) {
    let gitignore_path = current_dir.join(".gitignore");

    if !gitignore_path.exists() {
        show_warning("No .gitignore file found");
        return;
    }

    print!(
        "{} {} ",
        "[?]".cyan().bold(),
        "Add .cargorush.toml to .gitignore?".bold()
    );
    print!(
        "[{}Y{}/{}n{}] ",
        "".bold().green(),
        "".clear(),
        "".bold().red(),
        "".clear()
    );
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "" | "y" => {
            match std::fs::OpenOptions::new()
                .append(true)
                .open(&gitignore_path)
            {
                Ok(mut file) => {
                    if let Err(e) = writeln!(file, "\n# Added by cargo-rush\n.cargorush.toml") {
                        show_error(&format!("Failed to write to .gitignore: {}", e));
                        return;
                    }
                    show_success("Added .cargorush.toml to .gitignore");
                }
                Err(e) => show_error(&format!("Failed to open .gitignore: {}", e)),
            }
        }
        "n" => {
            show_warning("Skipping .gitignore modification");
            show_info("You can manually add '.cargorush.toml' to your .gitignore later");
        }
        _ => {
            show_error("Invalid input");
            show_info("Skipping .gitignore modification");
        }
    }
    println!();
}

fn populate_cargorush(current_dir: &std::path::Path) {
    let rush_file = current_dir.join(".cargorush.toml");

    match std::fs::OpenOptions::new().append(true).open(&rush_file) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "[rush.info]\nname = \"{}\"", env!("CARGO_PKG_NAME")) {
                show_error(&format!("Failed to write to .cargorush.toml: {}", e));
                return;
            }
        }
        Err(e) => show_error(&format!("Failed to open .cargorush.toml: {}", e)),
    }
}
