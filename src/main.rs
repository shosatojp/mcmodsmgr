#[macro_use]
extern crate prettytable;

mod cli;
mod commands;
mod lockfile;
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
use lockfile::Lockfile;
use std::{path::Path, process::exit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = cli::build_cli();
    let root_matches = app.get_matches_safe().unwrap_or_else(|err| {
        println!("{}", err.message);
        exit(0)
    });

    // join path
    let target_dir = Path::new(root_matches.value_of("dir").unwrap());
    let _lockfile_path = root_matches.value_of("lockfile").unwrap();
    let lockfile_path = if Lockfile::is_local_file(_lockfile_path) {
        let pathbuf = target_dir.join(_lockfile_path);
        match pathbuf.to_str() {
            Some(s) => s.to_string(),
            None => {
                eprintln!("failed to get path");
                return Err("failed to get path".to_string())?;
            }
        }
    } else {
        _lockfile_path.to_string()
    };

    let mut lockfile = Lockfile::new(&lockfile_path).await?;

    let lockfile_ref = match root_matches.value_of("lockfileref") {
        Some(path) => Some(Lockfile::new(path).await?),
        None => None,
    };

    match root_matches.subcommand() {
        ("search", Some(matches)) => {
            commands::search(matches.value_of("name").unwrap()).await?;
        }
        ("install", Some(matches)) => {
            commands::install(
                matches.value_of("name"),
                root_matches.value_of("mcversion"),
                root_matches.value_of("modloader"),
                matches
                    .value_of("fileid")
                    .and_then(|e| Some(e.parse::<usize>().unwrap())),
                matches.value_of("filename"),
                target_dir,
                !root_matches.is_present("full"),
                &mut lockfile,
                lockfile_ref,
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
        ("list", Some(_)) => {
            commands::list(&lockfile);
        }
        (_, _) => {
            eprintln!("Use \"{} -h\" to see the help", crate_name!());
            exit(1)
        }
    }
    Ok(())
}
