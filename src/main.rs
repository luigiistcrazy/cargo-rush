use clap::{Parser, CommandFactory};
use colored::*;
use std::env;
use std::io::{self, Write};

#[derive(Parser, Debug)]
enum CargoCli {
    Rush(RushArgs),
}

#[derive(clap::Args, Debug)]
#[command(version)]
struct RushArgs {
    /// Initialize cargo-rush in current directory
    #[arg(long)]
    init: bool,
}

fn main() {
    print_version_header();

    let args = std::env::args().collect::<Vec<_>>();

    if args.get(1).map(|s| s.as_str()) != Some("rush") {
        show_error("It is adviced to run cargo-rush with 'cargo rush'");
        std::process::exit(1);
    }

    match CargoCli::parse() {
        CargoCli::Rush(args) => {
            let current_dir = std::env::current_dir().expect("Failed to get current directory");

            check_cargo_toml(&current_dir);

            if args.init {
                handle_init(&current_dir);
            } else {
                check_cargorush_file(&current_dir);
                // After checks, print help for the 'rush' subcommand
                let mut cmd = CargoCli::command();
                if let Some(rush_cmd) = cmd.find_subcommand_mut("rush") {
                    rush_cmd.print_help().expect("Failed to print help");
                }
                std::process::exit(0);
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
    } else if current_dir.join(".cargorush.toml").exists() {
        // Set path for windows machines
        #[cfg(not(target_family = "unix"))]
        let cargorush_path = format!(
            ".cargorush.toml found at '{}\\.cargorush.toml'",
            env::current_dir().unwrap().display()
        );

        // Set path for non-windows machines
        let cargorush_path = format!(
            ".cargorush.toml found at '{}/.cargorush.toml'",
            env::current_dir().unwrap().display()
        );
        show_warning("cargo-rush already initialized");
        show_info(&cargorush_path);
    }
}

fn handle_init(current_dir: &std::path::Path) {
    let rush_file = current_dir.join(".cargorush.toml");

    if rush_file.exists() {
        // Set path for windows machines
        #[cfg(not(target_family = "unix"))]
        let cargorush_path = format!(
            ".cargorush.toml found at '{}\\.cargorush.toml'",
            env::current_dir().unwrap().display()
        );

        // Set path for non-windows machines
        let cargorush_path = format!(
            ".cargorush.toml found at '{}/.cargorush.toml'",
            env::current_dir().unwrap().display()
        );

        show_warning("cargo-rush is already active in this directory");
        show_info(&cargorush_path);
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
        "Append .cargorush.toml to .gitignore?".bold()
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
