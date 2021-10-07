#[macro_use]
extern crate prettytable;

mod cli;
mod util;
mod api {
    pub mod curseforge;
    pub mod curseforge_types;
}

use std::{process::exit, ptr::NonNull};

use api::curseforge;
pub use api::curseforge_types::{Addon, AddonFile};
use reqwest::Version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = cli::build_cli();

    if let Some(ref matches) = app.subcommand_matches("search") {
        let name = matches.value_of("name").unwrap();

        // search
        let mut addons = curseforge::search(name).await?;

        // filter by version
        if let Some(version) = matches.value_of("version") {
            addons.retain(|addon| {
                addon
                    .gameVersionLatestFiles
                    .iter()
                    .any(|gameVersionLatestFile| gameVersionLatestFile.gameVersion == version)
            })
        }

        // output
        if addons.len() > 0 {
            util::print_addons(&addons);
            println!("found {} mod(s)", addons.len());
        } else {
            eprintln!("not found");
            exit(1);
        }
    } else if let Some(ref matches) = app.subcommand_matches("install") {
        let slug = matches.value_of("name").unwrap();
        // search
        let mut addons = curseforge::search(slug).await?;
        if let Some(target) = addons.iter().find(|&addon| addon.slug == slug) {
            let file = match matches.value_of("version") {
                Some(version) => target
                    .gameVersionLatestFiles
                    .iter()
                    .find(|&file| file.gameVersion == version),
                None => target.gameVersionLatestFiles.iter().next(),
            };

            match file {
                Some(file) => {
                    let fileinfo = curseforge::get_fileinfo(target.id, file.projectFileId).await?;
                    util::download_file(&fileinfo.downloadUrl, &format!("{}", &fileinfo.fileName))
                        .await?;
                }
                None => panic!("file not found for version"),
            }
        } else {
            panic!("mod not found");
        }
    }
    Ok(())
}
