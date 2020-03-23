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
    Major(u32),
    MajorMinor(u32, u32),
    Specific(u32, u32, u32),
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
                let major_version = parts.get(0).unwrap().parse::<u32>()?;
                Ok(Self::Major(major_version))
            }
            2 => {
                let major_version = parts.get(0).unwrap().parse::<u32>()?;
                let minor_version = parts.get(1).unwrap().parse::<u32>()?;
                Ok(Self::MajorMinor(major_version, minor_version))
            }
            3 => {
                let major_version = parts.get(0).unwrap().parse::<u32>()?;
                let minor_version = parts.get(1).unwrap().parse::<u32>()?;
                let patch_version = parts.get(2).unwrap().parse::<u32>()?;
                Ok(Self::Specific(major_version, minor_version, patch_version))
            }
            _ => Err(InvalidVersionError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NodeVersion, InvalidVersionError};
    use std::convert::TryFrom;

    #[test]
    fn major_version() {
        assert_eq!(
            NodeVersion::try_from("10"),
            Ok(NodeVersion::Major(10)),
        );
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
        assert_eq!(
            NodeVersion::try_from("19.a"),
            Err(InvalidVersionError),
        );

        assert_eq!(
            NodeVersion::try_from(".1"),
            Err(InvalidVersionError),
        );

        assert_eq!(
            NodeVersion::try_from("hello world"),
            Err(InvalidVersionError),
        );
    }
}
