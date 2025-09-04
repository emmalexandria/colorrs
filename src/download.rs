use std::{
    fs,
    path::PathBuf,
};

use git2::Repository;
use tempfile::tempdir;

use crate::files::list_dir_files;

#[derive(Debug)]
pub enum DownloadErrorType {
    CloneFailed,
    InvalidURL,
    InvalidSubdir,
    TempDirFailure,
    IOError,
}

#[derive(Debug)]
pub struct DownloadError {
    pub e_type: DownloadErrorType,
    pub message: String,
}

impl DownloadError {
    fn new<S: ToString>(e_type: DownloadErrorType, message: S) -> Self {
        Self {
            e_type,
            message: message.to_string(),
        }
    }
}

pub fn download_patterns(url: String, pattern_dir: &PathBuf) -> Result<(), DownloadError> {
    let normalized = normalize_url(url)?;

    let dir = tempdir().map_err(|e| {
        DownloadError::new(
            DownloadErrorType::TempDirFailure,
            "Failed to create temporary directory",
        )
    })?;

    println!("Cloning {normalized}");
    let repo = Repository::clone(&normalized, dir.path()).map_err(|e| {
        DownloadError::new(
            DownloadErrorType::InvalidURL,
            format!("Failed to clone git repository with error {e}"),
        )
    })?;

    let mut pattern_path = dir.path().to_path_buf();
    pattern_path.push("patterns");
    let mut colorscript_path = dir.path().to_path_buf();
    colorscript_path.push("colorscripts");

    if pattern_path.exists() {
        let num = copy_contents_to_path(&pattern_path, pattern_dir)?;
        println!(
            "Copied {} patterns to {}",
            num,
            pattern_dir.to_string_lossy()
        );
        return Ok(());
    }
    if colorscript_path.exists() {
        let num = copy_contents_to_path(&colorscript_path, pattern_dir)?;
        println!(
            "Copied {} colorscripts to {}",
            num,
            pattern_dir.to_string_lossy()
        );
        return Ok(());
    }

    Err(DownloadError::new(
        DownloadErrorType::InvalidSubdir,
        "No pattern or colorscripts directory found in repository",
    ))
}

fn copy_contents_to_path(
    location: &PathBuf,
    destination: &PathBuf,
) -> Result<usize, DownloadError> {
    let files = list_dir_files(location).map_err(|e| {
        DownloadError::new(
            DownloadErrorType::IOError,
            format!("Failed to read temp directory with error {e}"),
        )
    })?;
    let mut count = files.len();
    files.iter().for_each(|f| {
        if let Some(name) = f.file_name() {
            let mut dest = destination.clone();

            dest.push(name);
            if dest.exists() || dest.with_extension("").exists() {
                println!(
                    "Not copying {} because a pattern with that name is already installed",
                    name.to_string_lossy()
                );
                count -= 1;
                return;
            }
            if let Err(e) = fs::copy(f, dest) {
                println!("Failed to copy {} with error {e}", name.to_string_lossy());
                count -= 1;
            }
        }
    });

    Ok(count)
}

fn normalize_url<S: ToString>(url: S) -> Result<String, DownloadError> {
    let url_str = url.to_string();

    if url_str.starts_with("http://") || url_str.starts_with("https://") {
        return Ok(url_str);
    }

    if let Some(slash) = url_str.find('/') {
        let author = &url_str[..slash];
        let name = &url_str[slash + 1..];

        if !author.is_empty() && !name.is_empty() {
            return Ok(format!("https://www.github.com/{}/{}", author, name));
        } else {
            return Err(DownloadError::new(
                DownloadErrorType::InvalidURL,
                "Detected Github type url, but no author or repository name found",
            ));
        }
    }

    Err(DownloadError::new(
        DownloadErrorType::InvalidURL,
        "URL is not HTTPS address and does not contain delimiting slash",
    ))
}

#[cfg(test)]
mod download_tests {
    use crate::download::normalize_url;

    #[test]
    fn test_normalize_url() {
        assert!(normalize_url("hello").is_err());
        assert!(normalize_url("hello/hi").is_ok_and(|u| u == "https://www.github.com/hello/hi"));
        assert!(normalize_url("https://www.hello.com").is_ok_and(|u| u == "https://www.hello.com"));
    }
}
