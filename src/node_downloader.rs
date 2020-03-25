use flate2::read::GzDecoder;
use reqwest::Url;
use std::env;
use std::error;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use tar::Archive;

#[derive(Debug, Clone)]
pub enum DownloadErrorKind {
    NoDownloadUrl,
    RequestError,
    NoDestination,
    CannotExtract,
}

#[derive(Debug, Clone)]
pub struct DownloadError(pub DownloadErrorKind);

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            DownloadErrorKind::NoDownloadUrl => write!(f, "No download URL"),
            DownloadErrorKind::RequestError => write!(f, "Error while making the request"),
            DownloadErrorKind::NoDestination => write!(f, "No destination for download"),
            DownloadErrorKind::CannotExtract => write!(f, "Cannot extract archive"),
        }
    }
}

impl error::Error for DownloadError {}
impl From<reqwest::Error> for DownloadError {
    fn from(_: reqwest::Error) -> Self {
        Self(DownloadErrorKind::RequestError)
    }
}
impl From<io::Error> for DownloadError {
    fn from(_: io::Error) -> Self {
        Self(DownloadErrorKind::NoDestination)
    }
}

pub struct NodeDownloader {}

impl NodeDownloader {
    /// Downloads the requested URL into a temporary directory and
    /// returns the path to the downloaded file.
    pub async fn download(download_url: Url) -> Result<PathBuf, DownloadError> {
        let file_name = {
            let name = &download_url
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("nen-node-download.tar.gz");
            env::temp_dir().join(name)
        };
        let mut destination = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file_name)?;

        let mut request = reqwest::get(download_url).await?.error_for_status()?;
        while let Some(chunk) = request.chunk().await? {
            destination.write(&chunk)?;
        }
        destination.flush()?;

        Ok(file_name)
    }

    /// Extracts the given file into the given path.
    pub fn extract(file: &File, path: &PathBuf) -> Result<(), io::Error> {
        let tar_gz = GzDecoder::new(file);
        let mut tar = Archive::new(tar_gz);
        tar.unpack(path)
    }
}
