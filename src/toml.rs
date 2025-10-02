use cargo_toml::Manifest;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use velvetio::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct CargoJetToml {
    name: String,
    path: PathBuf,
    remote_name: String,
    remote_url: String,
    remote_os: String,
}

pub fn init(proj_dir: &Path) {
    let cargo_jet_path = proj_dir.join("Cargojet.toml");
    let cargo_toml_path = proj_dir.join("Cargo.toml");
    let manifest = Manifest::from_path(cargo_toml_path).unwrap();
    let proj_name = manifest.package().name().to_string();
    let proj_dir = proj_dir.to_path_buf();

    // This fucking shit is so fucking shit and ugly fuck
    let remote_name = ask!("Please enter remote name (\"My Remote\")" => String);
    let remote_url = ask!("Please enter remote ssh address (\"user@host:port\")" => String);
    let remote_os = ask!("Please specify remote OS (\"Windows\", \"Linux\", \"macOS\")" => String);

    let cargo_jet_toml = CargoJetToml::new(proj_name, proj_dir, remote_name, remote_url, remote_os);

    println!("{:?}", cargo_jet_toml);
}

impl CargoJetToml {
    pub fn new(
        name: String,
        path: PathBuf,
        remote_name: String,
        remote_url: String,
        remote_os: String,
    ) -> CargoJetToml {
        CargoJetToml {
            name,
            path,
            remote_name,
            remote_url,
            remote_os,
        }
    }
}
