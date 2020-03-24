use std::convert::TryFrom;
use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct InvalidVersionError;

impl fmt::Display for InvalidVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Node.js version")
    }
}

impl error::Error for InvalidVersionError {}
impl From<std::num::ParseIntError> for InvalidVersionError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeVersion {
    Major(u64),
    MajorMinor(u64, u64),
    Specific(u64, u64, u64),
}

impl TryFrom<&str> for NodeVersion {
    type Error = InvalidVersionError;
    fn try_from(version: &str) -> Result<Self, Self::Error> {
        Self::try_from(String::from(version))
    }
}

impl TryFrom<String> for NodeVersion {
    type Error = InvalidVersionError;
    fn try_from(version: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = version.split(".").collect();
        match parts.len() {
            1 => {
                let major_version = parts[0].parse::<u64>()?;
                Ok(Self::Major(major_version))
            }
            2 => {
                let major_version = parts[0].parse::<u64>()?;
                let minor_version = parts[1].parse::<u64>()?;
                Ok(Self::MajorMinor(major_version, minor_version))
            }
            3 => {
                let major_version = parts[0].parse::<u64>()?;
                let minor_version = parts[1].parse::<u64>()?;
                let patch_version = parts[2].parse::<u64>()?;
                Ok(Self::Specific(major_version, minor_version, patch_version))
            }
            _ => Err(InvalidVersionError),
        }
    }
}

impl From<NodeVersion> for semver::Version {
    fn from(version: NodeVersion) -> Self {
        match version {
            NodeVersion::Major(major) => Self::from((major, 0u64, 0u64)),
            NodeVersion::MajorMinor(major, minor) => Self::from((major, minor, 0u64)),
            NodeVersion::Specific(major, minor, patch) => Self::from((major, minor, patch)),
        }
    }
}

impl PartialEq<NodeVersion> for semver::Version {
    fn eq(&self, node_version: &NodeVersion) -> bool {
        match node_version {
            NodeVersion::Major(major) => {
                self.major == *major
            }
            NodeVersion::MajorMinor(major, minor) => {
                self.major == *major && self.minor == *minor
            }
            NodeVersion::Specific(major, minor, patch) => {
                self.major == *major && self.minor == *minor && self.patch == *patch
            }
        }
    }
}
impl PartialEq<semver::Version> for NodeVersion {
    fn eq(&self, version: &semver::Version) -> bool {
        version == self
    }
}

#[cfg(test)]
mod tests {
    use super::{InvalidVersionError, NodeVersion};
    use std::convert::TryFrom;

    #[test]
    fn major_version() {
        assert_eq!(NodeVersion::try_from("10"), Ok(NodeVersion::Major(10)));
    }

    #[test]
    fn major_minor_version() {
        assert_eq!(
            NodeVersion::try_from("13.9"),
            Ok(NodeVersion::MajorMinor(13, 9)),
        );
    }

    #[test]
    fn specific_version() {
        assert_eq!(
            NodeVersion::try_from("13.9.0"),
            Ok(NodeVersion::Specific(13, 9, 0)),
        );
    }

    #[test]
    fn invalid_version() {
        assert_eq!(NodeVersion::try_from("19.a"), Err(InvalidVersionError));

        assert_eq!(NodeVersion::try_from(".1"), Err(InvalidVersionError));

        assert_eq!(
            NodeVersion::try_from("hello world"),
            Err(InvalidVersionError),
        );
    }
}
