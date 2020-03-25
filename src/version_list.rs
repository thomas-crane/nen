use regex::Regex;
use reqwest;
use reqwest::Url;
use scraper::{Html, Selector};
use semver::Version;

use crate::node_version::NodeVersion;

const NODE_URL: &'static str = "https://nodejs.org/dist/";

pub struct VersionList {
    versions: Vec<Version>,
}

impl<'a> VersionList {
    // TODO(thomas.crane): replace error with custom error type.
    pub async fn create() -> Result<Self, Box<dyn std::error::Error>> {
        let body = reqwest::get(NODE_URL).await?.text().await?;
        let body = Html::parse_fragment(&body);
        let selector = Selector::parse("a").unwrap();
        let version_pattern = Regex::new(r"v(\d+\.\d+\.\d+)/")?;
        let mut versions: Vec<Version> = body
            .select(&selector)
            .filter_map(|item| item.value().attr("href"))
            .filter(|version| version_pattern.is_match(version))
            .map(|version| &version[1..version.len() - 1])
            .filter_map(|version| Version::parse(version).ok())
            .collect();
        // sort and reverse the list so that when we are
        // translating something like "12.x.x" into the most
        // recent version, we can just take the first
        // version that has a major version of 12.
        versions.sort_unstable();
        versions.reverse();
        Ok(Self { versions })
    }

    pub fn has_version(&self, version: &Version) -> bool {
        self.versions.contains(version)
    }

    pub fn latest_version_of(&self, node_version: &NodeVersion) -> Option<&Version> {
        self.versions.iter().find(|v| *v == node_version)
    }

    pub fn download_url(&self, node_version: &NodeVersion) -> Option<Url> {
        let version = self.latest_version_of(node_version)?;

        let mut version_string = String::from("v");
        version_string.push_str(&version.to_string());

        let mut tarball_string = String::from("node-");
        tarball_string.push_str(&version_string);
        tarball_string.push_str(".tar.gz");

        version_string.push('/');

        Url::parse(NODE_URL).ok()?
            .join(&version_string).ok()?
            .join(&tarball_string).ok()
    }
}
