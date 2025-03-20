# !!WIP!! NOT READY FOR USE !!WIP!!
# 🚀 cargo-rush 💻🔄🖥️

**cargo-rush** allows you to build, test, and run your Rust code on a remote machine effortlessly. It keeps everything in sync between your local and remote environments using **rsync**. **Cargo-rush** is particularly useful if you are working on a low-spec device and want to compile your Rust projects quicker using a powerful remote machine.

## Features

- **Seamless Remote Execution** – Build, test, and run your Rust projects on a remote machine as if they were local.
- **Automatic Syncing** – Keeps files in sync between your local and remote machines via **rsync**.
- **Effortless Setup** – Simple to install and initialize.

## Installation

To install **cargo-rush**, clone the repository and install it using `cargo`:

```sh
git clone https://github.com/luigiistcrazy/cargo-rush
cd cargo-rush
cargo install --path .
```

## Usage

Run `cargo rush` with the desired arguments:

```sh
❯❯❯ cargo rush

🚀 cargo-rush v0.0.3-indev

Build and run your Rust projects effortlessly on a remote machine! 🦀⚡

Usage: cargo rush [OPTIONS]

Options:
      --init     Initialize cargo-rush in current directory
  -h, --help     Print help
  -V, --version  Print version
```

### Available Arguments:

| Argument       | Description                            |
|---------------|--------------------------------|
| `--init`      | Initialize cargo-rush in the current directory |
| `-h, --help`  | Display help information       |
| `-V, --version` | Show the version number |

### Example Usage:

```sh
cargo rush --init          # Initialize cargo-rush in your project 
cargo rush build           # Build your project remotely (WIP)
cargo rush sync            # Synchronizes files between local and remote (WIP)
cargo rush sync --dry-run  # Simulates the sync without making changes. (WIP)
```
## Configuration

After initialization, cargo-rush sets up the necessary configuration files to define your remote machine, syncing paths, and build settings. **Cargo-rush** has git-like syntax making configuration easy.

### (WIP) Example Configuration:

```sh
cargo rush remote add <name> <user@host>          # Adds a new remote host.
cargo rush remote remove <name>                   # Removes an existing remote.
cargo rush remote rename <old-name> <new-name>    # Renames a remote.
cargo rush remote list                            # Shows all configured remotes.
```

and much more...

## WIP Overview

**cargo-rush** is heavy WIP and not ready for usage. The first official alpha release is expected once
[Project Setup](https://github.com/luigiistcrazy/cargo-rush/new/master?filename=README.md#project-setup),
[Remote Management](https://github.com/luigiistcrazy/cargo-rush/new/master?filename=README.md#remote-management),
[File Synchroniation](https://github.com/luigiistcrazy/cargo-rush/new/master?filename=README.md#file-synchronization),
and [Remote Execution](https://github.com/luigiistcrazy/cargo-rush/new/master?filename=README.md#remote-execution) are implemented to a basic extend.

### Project Setup
- [x] Initialize the project with `cargo rush init`
- [x] Append .cargorush to .gitignore
- [ ] Append project details to .cargorush

### Remote Management
- [ ] Add remote using `cargo rush remote add`
- [ ] Remove remote using `cargo rush remote remove`
- [ ] List all remotes with `cargo rush remote list`

### File Synchronization
- [ ] Implement `cargo rush sync`
- [ ] Test `cargo rush sync --dry-run`
- [ ] Implement automatic file synchronization

### Remote Execution
- [ ] Implement `cargo rush build`
- [ ] Implement `cargo rush run`
- [ ] Implement `cargo rush test`

### Configuration
- [ ] Add configuration options with `cargo rush config set`
- [ ] View configuration with `cargo rush config get`
- [ ] List all configurations with `cargo rush config list`

### Logs & Status
- [ ] Implement `cargo rush status`
- [ ] Implement `cargo rush logs`

### SSH & Remote Commands
- [ ] Implement SSH connection with `cargo rush ssh`
- [ ] Implement remote execution with `cargo rush exec`

## Contributing
Contributions are welcome! Feel free to submit issues or pull requests on GitHub.
