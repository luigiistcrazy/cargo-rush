use std::fs;
use std::io;

pub fn cli() {
    let dir_name = "test";
    println!("Initializing project...");
    match fs::create_dir_all(dir_name) {
        Ok(_) => {
            println!("Successfully created directory: '{}'", dir_name);
        }
        Err(e) => {
            if e.kind() == io::ErrorKind::AlreadyExists {
                println!("Directory '{}' already exists.", dir_name);
            } else {
                eprintln!("Failed to create directory '{}': {}", dir_name, e);
            }
        }
    }
}
