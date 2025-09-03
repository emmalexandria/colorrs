use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pattern {
    colors: HashMap<String, String>,
    pattern: String,
}

pub fn read_toml_pattern(path: &Path) -> Pattern {
    let content = fs::read_to_string(path).unwrap();
    let pat: Pattern = toml::from_str(&content).unwrap();

    pat
}

pub fn get_pattern_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("colorrs");
        path
    })
}

pub fn print_pattern(path: &Path) {
    let pattern = read_toml_pattern(path);

    println!("{}", pattern.pattern)
}
