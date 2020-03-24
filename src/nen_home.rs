use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone)]
/// An error which occurred when getting the home path for nen.
pub struct PathError;

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot get home path")
    }
}
impl error::Error for PathError {}

/// A representation of the nen home directory. Provides useful
/// functions for interacting with the folders in the nen home.
pub struct NenHome {
    home_path: PathBuf,
    bin_path: PathBuf,
    env_path: PathBuf,
}

impl NenHome {
    /// Creates a new nen home. This will fail if there is
    /// no `$NEN_HOME` *and* the `$HOME` variable cannot be read.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home_path = match env::var("NEN_HOME") {
            Ok(path) => Ok(PathBuf::from(path)),
            Err(_) => dirs::home_dir().ok_or(PathError).map(|p| p.join(".nen")),
        }?;
        let bin_path = home_path.join("binaries");
        let env_path = home_path.join("environments");
        Ok(Self {
            home_path,
            bin_path,
            env_path,
        })
    }

    /// Checks if all of the necessary folders exist for this nen home.
    pub fn is_valid_home(&self) -> bool {
        fs::metadata(&self.home_path)
            .and_then(|_| fs::metadata(&self.bin_path))
            .and_then(|_| fs::metadata(&self.env_path))
            .is_ok()
    }

    /// Creates any missing folders which are required for this to be a nen home.
    pub fn init_home(&self) -> Result<(), io::Error> {
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
