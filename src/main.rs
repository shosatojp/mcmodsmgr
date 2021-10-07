#[macro_use]
extern crate prettytable;

mod cli;
mod commands;
mod util;
mod tests {
    pub mod util;
}
mod api {
    pub mod curseforge;
    pub mod curseforge_types;
}

pub use api::curseforge_types::{Addon, AddonFile};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = cli::build_cli();
    let root_matches = app.get_matches_mut();

    match root_matches.subcommand() {
        Some(("search", matches)) => {
            commands::search(matches.value_of("name").unwrap()).await?;
        }
        Some(("install", matches)) => {
            commands::install(
                matches.value_of("name").unwrap(),
                root_matches.value_of("version"),
                root_matches.value_of("modloader"),
                matches
                    .value_of("fileid")
                    .and_then(|e| Some(e.parse::<usize>().unwrap())),
                matches.value_of("filename"),
                !root_matches.is_present("full"),
            )
            .await?;
        }
        Some(("describe", matches)) => {
            commands::describe(
                matches.value_of("name").unwrap(),
                root_matches.value_of("version"),
                root_matches.value_of("modloader"),
                !root_matches.is_present("full"),
            )
            .await?;
        }
        Some((_, _)) => {
            // unreachable
        }
        None => {
            let _ = app.print_help();
        }
    }
    Ok(())
}
