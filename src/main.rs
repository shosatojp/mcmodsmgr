#[macro_use]
extern crate prettytable;

mod cli;
mod commands;
mod util;
mod api {
    pub mod curseforge;
    pub mod curseforge_types;
}

use core::panic;
use std::process::exit;

use api::curseforge;
use api::curseforge_types::modloader_type;
pub use api::curseforge_types::{Addon, AddonFile};
use reqwest::Version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = cli::build_cli();

    if let Some(ref matches) = app.subcommand_matches("search") {
        commands::search(matches.value_of("name").unwrap()).await?;
    } else if let Some(ref matches) = app.subcommand_matches("install") {
        commands::install(
            matches.value_of("name").unwrap(),
            app.value_of("version"),
            app.value_of("modloader"),
            matches
                .value_of("fileid")
                .and_then(|e| Some(e.parse::<usize>().unwrap())),
            matches.value_of("filename"),
            !app.is_present("full"),
        )
        .await?;
    } else if let Some(ref matches) = app.subcommand_matches("describe") {
        commands::describe(
            matches.value_of("name").unwrap(),
            app.value_of("version"),
            app.value_of("modloader"),
            !app.is_present("full"),
        )
        .await?;
    }
    Ok(())
}
