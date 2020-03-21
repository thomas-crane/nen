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
impl From<io::Error> for PathError {
    fn from(_: io::Error) -> Self {
        Self
    }
}

/// Gets the home path for nen. This will either be the `$NEN_HOME` env var, or `$HOME/.nen`.
/// If the path does not exist yet, it will be created.
///
/// Returns `Ok` with the path, or `Err` with a `PathError` if there is some problem.
pub fn home_path() -> Result<PathBuf, PathError> {
    let path = match env::var("NEN_HOME") {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(_) => dirs::home_dir().ok_or(PathError).map(|p| p.join(".nen")),
    }?;
    if let Err(fs_err) = fs::metadata(&path) {
        if fs_err.kind() == io::ErrorKind::NotFound {
            fs::create_dir(&path)?;
        }
    }
    Ok(path)
}
