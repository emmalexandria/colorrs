#[derive(Debug)]
enum DownloadErrorType {
    NoRepository,
    InvalidURL,
    InvalidSubdir,
}

#[derive(Debug)]
struct DownloadError {
    e_type: DownloadErrorType,
    message: String,
}

impl DownloadError {
    fn new<S: ToString>(e_type: DownloadErrorType, message: S) -> Self {
        Self {
            e_type,
            message: message.to_string(),
        }
    }
}

pub fn download_patterns(url: String) -> Result<(), DownloadError> {
    let normalized = normalize_url(url)?;

    Ok(())
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
