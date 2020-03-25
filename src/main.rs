mod nen_home;
mod node_downloader;
mod node_version;
mod platform_string;
mod version_list;

use crate::nen_home::NenHome;
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
        println!("{} is a valid Node.js version", version);
        let home = NenHome::new()?;
        let home = home.init_home()?;
        println!(
            "Downloading Node.js v{}",
            version_list.latest_version_of(&version).unwrap()
        );
        home.download_node_version(&version, &version_list).await?;
        println!("Done");
    } else {
        println!("{} is not a valid Node.js version", version);
    }
    Ok(())
}
