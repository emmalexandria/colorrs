use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum PatternErrorType {
    InvalidTOML,
    FileDoesNotExist,
    IOError,
}

impl Display for PatternErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTOML => write!(f, "InvalidTOML"),
            Self::FileDoesNotExist => write!(f, "FileDoesNotExist"),
            Self::IOError => write!(f, "IOError"),
        }
    }
}

#[derive(Debug)]
pub struct PatternError {
    e_type: PatternErrorType,
    message: String,
}

impl PatternError {
    fn new<S: ToString>(e_type: PatternErrorType, msg: S) -> Self {
        Self {
            e_type,
            message: msg.to_string(),
        }
    }
}

impl Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.e_type, self.message)
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pattern {
    colors: HashMap<String, String>,
    pattern: String,
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.draw())
    }
}

impl Pattern {
    fn draw(&self) -> String {
        let mut new_pattern = self.pattern.clone();

        self.colors.iter().for_each(|(k, v)| {
            let pattern = format!("{{{}}}", k);
            new_pattern = new_pattern.replace(&pattern, &make_escape_code(v));
        });

        new_pattern = new_pattern.replace("{reset}", &make_escape_code("0"));

        new_pattern
    }
}

fn make_escape_code(inner: &str) -> String {
    format!("\x1b[{inner}m")
}

pub fn get_pattern_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("colorrs");
        path
    })
}

pub fn print_pattern(path: &Path) -> Result<(), PatternError> {
    if path.is_file() {
        if let Some(p) = path.extension()
            && p == "toml"
        {
            print_toml_pattern(path)?;
        } else {
            print_shell_pattern(path)?;
        }
    } else {
        return Err(PatternError::new(
            PatternErrorType::FileDoesNotExist,
            format!("Provided path {} is not a file", path.to_string_lossy()),
        ));
    }

    Ok(())
}

fn print_toml_pattern(path: &Path) -> Result<(), PatternError> {
    let content = fs::read_to_string(path).map_err(|e| {
        PatternError::new(
            PatternErrorType::IOError,
            format!("Error {} reading {}", e, path.to_string_lossy()),
        )
    })?;

    let pattern = toml::from_str::<Pattern>(&content).map_err(|e| {
        PatternError::new(
            PatternErrorType::InvalidTOML,
            format!(
                "TOML file {} invalid. Error: {}",
                path.to_string_lossy(),
                e.message()
            ),
        )
    })?;

    print!("{}", pattern.draw());

    Ok(())
}

fn print_shell_pattern(path: &Path) -> Result<(), PatternError> {
    let output = std::process::Command::new(path).output().unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
