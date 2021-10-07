use std::process::exit;

use crate::api::curseforge;
use crate::api::curseforge_types::modloader_type;
pub use crate::api::curseforge_types::{Addon, AddonFile};
use crate::util;
use reqwest::Version;

pub async fn search(name: &str) -> Result<(), String> {
    // search & filter
    let mut addons: Vec<Addon> = curseforge::search(name).await.or(Err("failed to search"))?;

    // output
    if addons.len() > 0 {
        util::print_addons(&addons);
        println!("found {} mod(s)", addons.len());
        return Ok(());
    } else {
        eprintln!("mod not found");
        return Err("mod not found".to_string());
    }
}

pub async fn install(
    slug: &str,
    version: Option<&str>,
    modloader: Option<&str>,
    fileid: Option<usize>,
    filename: Option<&str>,
) -> Result<(), String> {
    // search
    let mut target = match util::search_multiple_candidates(slug).await {
        Ok(value) => value,
        Err(_) => {
            eprintln!("mod not found");
            return Err("mod not found".to_string());
        }
    };

    // filter by...
    let orig_files = curseforge::get_files(target.id)
        .await
        .or(Err("failed to get files"))?;
    let mut files =
        util::filter_addonfiledetails_by(&orig_files, version, modloader, fileid, filename);

    match files.len() {
        1 => {
            let file = files.first().unwrap();
            // let fileinfo = curseforge::get_fileinfo(target.id, file.id)
            //     .await
            //     .or(Err("failed to fetch fileinfo"))?;
            eprintln!("downloading {} ...", file.fileName);
            util::download_file(&file.downloadUrl, &format!("{}", &file.fileName)).await?;
            return Ok(());
        }
        0 => {
            eprintln!("file not found for version");
            return Err("file not found for version".to_string());
        }
        _ => {
            eprintln!("multiple candidates found");
            util::print_files(&files);
            return Err("multiple candidates found".to_string());
        }
    }
}

pub async fn describe(
    slug: &str,
    version: Option<&str>,
    modloader: Option<&str>,
) -> Result<(), String> {
    // search
    let mut target = match util::search_multiple_candidates(slug).await {
        Ok(value) => value,
        Err(_) => {
            eprintln!("mod not found");
            return Err("mod not found".to_string());
        }
    };

    // filter by...
    let orig_files = curseforge::get_files(target.id)
        .await
        .or(Err("failed to get files"))?;
    let mut files = util::filter_addonfiledetails_by(&orig_files, version, modloader, None, None);

    // list files
    println!("{} (id:{})", target.name, target.id);
    println!();
    println!("{} downloads", target.downloadCount);
    println!("{}", target.websiteUrl);
    println!();
    println!("files:");
    util::print_files(&files);
    Ok(())
}
