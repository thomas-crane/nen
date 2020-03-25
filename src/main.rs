mod nen_home;
mod node_downloader;
mod node_version;
mod platform_string;
mod version_list;

use crate::nen_home::NenHome;
use crate::node_version::NodeVersion;
use std::convert::TryFrom;
use clap::{App, Arg, AppSettings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the commands.
    let matches = App::new("nen")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(App::new("init").about("Initialise your nen home."))
        .subcommand(
            App::new("new")
                .about("Create a new environment.")
                .arg(
                    Arg::with_name("name")
                        .help("The name of the environment.")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("version")
                        .help("The node version to use.")
                        .index(2)
                        .required(true),
                ),
        ).get_matches();

    // check for nen init
    if matches.subcommand_name() == Some("init") {
        let home = NenHome::new()?;
        if home.is_valid_home() {
            println!("Your home has already been initialised.");
            return Ok(());
        } else {
            home.init_home()?;
            println!("Successfully initialised nen home.");
            return Ok(());
        }
    }
    // create the home
    let home = NenHome::new()?;
    if !home.is_valid_home() {
        let path = home.home_path();
        println!(
            "Your current nen home ({}) is not initialised. Run `nen init` to do this.",
            path.to_str().unwrap_or("unknown"),
        );
        return Ok(());
    }
    let home = home.validate()?;

    match matches.subcommand_name() {
        Some("new") => {
            let matches = matches.subcommand_matches("new").unwrap();
            let name = matches.value_of("name").unwrap();
            let version = matches.value_of("version").unwrap();
            let version = NodeVersion::try_from(version)?;
            home.create_env(&String::from(name), &version).await?;
            Ok(())
        }
        _ => unreachable!()
    }
}
