use crate::types::OS;
use cargo_toml::Manifest;
use std::path::{Path, PathBuf};

struct CargoJetToml {
    name: String,
    path: PathBuf,
    remote_name: String,
    remote_url: String,
    remote_os: OS,
}

pub fn init(proj_dir: &Path) {
    let cargo_jet_path = proj_dir.join("Cargojet.toml");
    let cargo_toml_path = proj_dir.join("Cargo.toml");
    let manifest = Manifest::from_path(cargo_toml_path).unwrap();
    let proj_name = manifest.package().name().to_string();
}
