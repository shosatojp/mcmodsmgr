#[macro_use]
extern crate prettytable;

mod cli;
mod commands;
mod util;
mod tests {
    pub mod util;
    mod api {
        pub mod curseforge;
    }
}
mod api {
    pub mod curseforge;
    pub mod curseforge_types;
}

pub use api::curseforge_types::{Addon, AddonFile};
use clap::crate_name;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = cli::build_cli();
    let root_matches = app.get_matches_safe().unwrap_or_else(|err| {
        println!("{}", err.message);
        exit(0)
    });

    match root_matches.subcommand() {
        ("search", Some(matches)) => {
            commands::search(matches.value_of("name").unwrap()).await?;
        }
        ("install", Some(matches)) => {
            commands::install(
                matches.value_of("name").unwrap(),
                root_matches.value_of("mcversion"),
                root_matches.value_of("modloader"),
                matches
                    .value_of("fileid")
                    .and_then(|e| Some(e.parse::<usize>().unwrap())),
                matches.value_of("filename"),
                !root_matches.is_present("full"),
            )
            .await?;
        }
        ("describe", Some(matches)) => {
            commands::describe(
                matches.value_of("name").unwrap(),
                root_matches.value_of("mcversion"),
                root_matches.value_of("modloader"),
                !root_matches.is_present("full"),
            )
            .await?;
        }
        (_, _) => {
            eprintln!("Use \"{} -h\" to see the help", crate_name!());
            exit(1)
        }
    }
    Ok(())
}
