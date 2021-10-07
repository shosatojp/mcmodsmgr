#[macro_use]
extern crate prettytable;

mod cli;
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
        let name = matches.value_of("name").unwrap();

        // search & filter
        let mut addons: Vec<Addon> = curseforge::search(name).await?;
        addons.retain(|addon| {
            let mut files = util::filter_addonfiles_by(
                &addon.gameVersionLatestFiles,
                app.value_of("version"),
                app.value_of("modloader"),
                None,
                None,
            );
            !files.is_empty()
        });

        // output
        if addons.len() > 0 {
            util::print_addons(&addons);
            println!("found {} mod(s)", addons.len());
        } else {
            eprintln!("mod not found");
            exit(1);
        }
    } else if let Some(ref matches) = app.subcommand_matches("install") {
        let slug = matches.value_of("name").unwrap();
        // search
        let mut addons = curseforge::search(slug).await?;
        if let Some(target) = addons.iter().find(|&addon| addon.slug == slug) {
            let mut files = util::filter_addonfiles_by(
                &target.gameVersionLatestFiles,
                app.value_of("version"),
                app.value_of("modloader"),
                matches
                    .value_of("fileid")
                    .and_then(|e| Some(e.parse::<usize>().unwrap())),
                matches.value_of("filename"),
            );

            match files.len() {
                1 => {
                    let file = files.first().unwrap();
                    let fileinfo = curseforge::get_fileinfo(target.id, file.projectFileId).await?;
                    eprintln!("downloading {} ...", fileinfo.fileName);
                    util::download_file(&fileinfo.downloadUrl, &format!("{}", &fileinfo.fileName))
                        .await?;
                }
                0 => {
                    eprintln!("file not found for version");
                    exit(1);
                }
                _ => {
                    eprintln!("multiple candidates found");
                    util::print_files(&files);
                    exit(1);
                }
            }
        } else {
            eprintln!("mod not found");
            exit(1);
        }
    } else if let Some(ref matches) = app.subcommand_matches("describe") {
        let slug = matches.value_of("name").unwrap();
        // search
        let mut addons = curseforge::search(slug).await?;
        if let Some(target) = addons.iter().find(|&addon| addon.slug == slug) {
            // filter by...
            let mut files = util::filter_addonfiles_by(
                &target.gameVersionLatestFiles,
                app.value_of("version"),
                app.value_of("modloader"),
                None,
                None,
            );

            // list files
            println!("{} (id:{})", target.name, target.id);
            println!();
            println!("{} downloads", target.downloadCount);
            println!("{}", target.websiteUrl);
            println!();
            println!("files:");
            util::print_files(&files);
        } else {
            eprintln!("mod not found");
            exit(1);
        }
    }
    Ok(())
}
