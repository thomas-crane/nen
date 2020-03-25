mod nen_home;
mod node_downloader;
mod node_version;
mod platform_string;
mod version_list;

use crate::nen_home::NenHome;
use crate::node_version::NodeVersion;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arg_iter = std::env::args().skip(1);
    let name = arg_iter.next().expect("Provide a name");
    let version = arg_iter.next().expect("Provide a version");

    let version = NodeVersion::try_from(version)?;

    let home = NenHome::new()?;
    let home = home.init_home()?;

    home.create_env(&name, &version).await?;
    Ok(())
}
