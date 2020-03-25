use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::node_downloader::{DownloadError, DownloadErrorKind, NodeDownloader};
use crate::node_version::NodeVersion;
use crate::version_list::VersionList;

#[derive(Debug, Clone)]
/// An error which occurred when getting the home path for nen.
pub struct PathError;

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot get home path")
    }
}
impl error::Error for PathError {}

struct HomeLayout {
    home_path: PathBuf,
    bin_path: PathBuf,
    env_path: PathBuf,
}

impl HomeLayout {
    fn new(home_path: PathBuf) -> Self {
        let bin_path = home_path.join("binaries");
        let env_path = home_path.join("environments");
        Self {
            home_path,
            bin_path,
            env_path,
        }
    }

    fn validate(&self) -> Result<(), io::Error> {
        fs::metadata(&self.home_path)?;
        fs::metadata(&self.bin_path)?;
        fs::metadata(&self.env_path)?;
        Ok(())
    }

    fn create_dirs(&self) -> Result<(), io::Error> {
        for path in &[&self.home_path, &self.env_path, &self.bin_path] {
            if let Err(err) = fs::create_dir(path) {
                // if the path already exists, that's fine.
                // otherwise return the error now.
                if err.kind() != io::ErrorKind::AlreadyExists {
                    return Err(err);
                }
            }
        }
        Ok(())
    }
}

/// A representation of the nen home directory. Provides useful
/// functions for interacting with the folders in the nen home.
pub struct NenHome {
    home: HomeLayout,
}

impl NenHome {
    /// Creates a new nen home. This will fail if there is
    /// no `$NEN_HOME` *and* the `$HOME` variable cannot be read.
    pub fn new() -> Result<Self, PathError> {
        let home_path = match env::var("NEN_HOME") {
            Ok(path) => Ok(PathBuf::from(path)),
            Err(_) => dirs::home_dir().ok_or(PathError).map(|p| p.join(".nen")),
        }?;

        Ok(Self {
            home: HomeLayout::new(home_path),
        })
    }

    /// Checks if all of the necessary folders exist for this nen home.
    pub fn is_valid_home(&self) -> bool {
        self.home.validate().is_ok()
    }

    /// Creates any missing folders which are required for this to be a nen home.
    pub fn init_home(self) -> Result<ValidNenHome, io::Error> {
        self.home.create_dirs()?;
        Ok(ValidNenHome { home: self.home })
    }
}

#[derive(Debug, Clone)]
/// An error which occurred when getting a node version.
pub struct ReadVersionError;

impl fmt::Display for ReadVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot read node version.")
    }
}
impl error::Error for ReadVersionError {}
impl From<io::Error> for ReadVersionError {
    fn from(_: io::Error) -> Self {
        Self
    }
}

pub struct ValidNenHome {
    home: HomeLayout,
}

impl ValidNenHome {
    pub async fn create_env(
        &self,
        name: &String,
        version: &NodeVersion,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let env_path = self.home.env_path.join(name);
        // create the dir, this will fail if it already exists.
        fs::create_dir(&env_path)?;

        // create the other dirs
        fs::create_dir(&env_path.join(".npm-global"))?;
        let bin_path = env_path.join("bin");

        // make sure the requested node version is installed.
        if !self.has_node_version(&version) {
            let version_list = VersionList::create().await?;
            self.download_node_version(&version, &version_list).await?;
        }

        // get the node version dir and create the symlinks.
        let node_dir = self.node_version_dir(&version)?;
        let node_bin_dir = node_dir.path().join("bin");
        symlink::symlink_dir(node_bin_dir.canonicalize()?, bin_path)?;

        Ok(())
    }

    pub async fn download_node_version(
        &self,
        version: &NodeVersion,
        version_list: &VersionList,
    ) -> Result<(), DownloadError> {
        let download_url = version_list
            .download_url(version)
            .ok_or(DownloadError(DownloadErrorKind::NoDownloadUrl))?;

        // download the binary.
        let node_archive_path = NodeDownloader::download(download_url).await?;
        let node_archive = File::open(node_archive_path)?;

        // extract it.
        let node_path = self.home.bin_path.join(version.to_string());
        NodeDownloader::extract(&node_archive, &node_path)
            .map_err(|_| DownloadError(DownloadErrorKind::CannotExtract))?;

        Ok(())
    }

    fn has_node_version(&self, version: &NodeVersion) -> bool {
        Path::new(&self.home.bin_path.join(version.to_string())).exists()
    }

    fn node_version_dir(&self, version: &NodeVersion) -> Result<DirEntry, ReadVersionError> {
        let mut entries = fs::read_dir(&self.home.bin_path.join(version.to_string()))?;

        if let Some(dir) = entries.next() {
            Ok(dir?)
        } else {
            Err(ReadVersionError)
        }
    }
}
