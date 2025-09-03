use std::{
    fs, os,
    path::{Path, PathBuf},
};

pub fn list_dir_files(dir: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_file() {
                            files.push(path)
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        Err(e) => return Err(e),
    }

    Ok(files)
}

fn validate_file(path: PathBuf) -> bool {
    if path.extension().unwrap() == "toml" {
        return true;
    }

    false
}
