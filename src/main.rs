mod node_version;
mod path;
mod version_list;

use crate::node_version::NodeVersion;
use crate::version_list::VersionList;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("Provide a version to check.");
    let version = NodeVersion::try_from(arg)?;
    let version_list = VersionList::create().await?;
    if version_list.has_version(&version.clone().into()) {
        println!("{:?} is a valid Node.js version", version);
    } else {
        println!("{:?} is not a valid Node.js version", version);
    }
    let latest = version_list.latest_version_of(&version);
    if let Some(v) = latest {
        println!("The latest version of {:?} is '{}'", &version, v);
    }
    Ok(())
}
