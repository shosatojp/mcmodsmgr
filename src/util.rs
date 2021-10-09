use clap::crate_name;

use crate::api::curseforge;
use crate::api::curseforge_types::AddonFileDetail;
use crate::Addon;
use core::result::Result;
use std::collections::HashSet;
use std::io::Write;

pub async fn download_file(url: &str, path: &str) -> Result<(), String> {
    let mut file = std::fs::File::create(path).or(Err("failed to open file"))?;
    let content = reqwest::get(url)
        .await
        .or(Err("failed to request"))?
        .bytes()
        .await
        .or(Err("failed to get body"))?;
    let _ = file.write(&content);
    Ok(())
}

pub async fn download_file_chunked(url: &str, path: &str) -> Result<(), String> {
    let mut file = std::fs::File::create(path).or(Err("failed to open file"))?;
    let mut res = reqwest::get(url).await.or(Err("failed to request"))?;
    if !res.status().is_success() {
        return Err("an error occured on get file".to_string());
    }
    while let Some(chunk) = res.chunk().await.or(Err("failed to get chunk"))? {
        let _ = file.write(&chunk);
    }
    Ok(())
}

pub async fn download_file_chunked_with_temp(url: &str, path: &str) -> Result<(), String> {
    loop {
        let mut temp_path = std::env::temp_dir();
        let rand_value: usize = rand::random();
        temp_path.push(format!("{}.{}.tmp", crate_name!(), rand_value));

        if temp_path.exists() {
            continue;
        } else {
            download_file_chunked(url, temp_path.to_str().ok_or("failed to get path str")?).await?;
            std::fs::copy(&temp_path, path).or(Err("failed to copy"))?;
            std::fs::remove_file(&temp_path).or(Err("failed to remove"))?;
            break;
        }
    }
    Ok(())
}

#[tokio::test]
async fn test_download_file_chunked_with_temp() {
    let url = "https://avatars.githubusercontent.com/u/40783705";
    let path = "test.png";
    match download_file_chunked_with_temp(url, path).await {
        Ok(_) => println!("ok"),
        Err(err) => eprintln!("{:?}", err),
    }
}

pub fn print_addons(addons: &Vec<Addon>) {
    let mut table = prettytable::Table::new();
    table.add_row(row!["Mod Id", "Display Name", "Short Name"]);
    for addon in addons {
        table.add_row(row![addon.id, addon.name, format!("{}", addon.slug)]);
    }
    table.printstd();
}

const MODLOADERS: [&str; 2] = ["forge", "fabric"];

/// get a modloader type from gameVersions.
pub fn game_version_tags_to_modloader(versions: &Vec<String>) -> Option<String> {
    for version in versions {
        let lower = version.to_lowercase();
        if MODLOADERS.contains(&lower.as_str()) {
            return Some(lower);
        }
    }
    None
}

pub fn print_files(files: &Vec<AddonFileDetail>) {
    let mut table = prettytable::Table::new();
    table.add_row(row!["File Id", "Version", "Mod Loader", "File Name"]);
    for file in files {
        let mod_loader = game_version_tags_to_modloader(&file.gameVersion);
        let versions: Vec<String> = file
            .gameVersion
            .iter()
            .filter(|&s| !MODLOADERS.contains(&s.to_lowercase().as_str()))
            .cloned()
            .collect();

        table.add_row(row![
            file.id,
            versions.join(", "),
            mod_loader.unwrap_or("-".to_string()),
            file.fileName,
        ]);
    }
    table.printstd();
}

pub fn filter_addonfiledetails_by(
    addonfiles: &Vec<AddonFileDetail>,
    version: Option<&str>,
    modloader: Option<&str>,
    fileid: Option<usize>,
    filename: Option<&str>,
) -> Vec<AddonFileDetail> {
    // filter by version
    return addonfiles
        .iter()
        // filter by fileid
        .filter(|&file| {
            if let Some(fileid) = fileid {
                file.id == fileid
            } else {
                true
            }
        })
        // filter by filename
        .filter(|&file| {
            if let Some(filename) = filename {
                file.fileName == filename
            } else {
                true
            }
        })
        // filter by version
        .filter(|&file| {
            if let Some(version) = version {
                file.gameVersion.contains(&version.to_string())
            } else {
                true
            }
        })
        // filter by modloader
        .filter(|&file| {
            if let Some(modloader) = modloader {
                match game_version_tags_to_modloader(&file.gameVersion) {
                    Some(val) => val == modloader,
                    None => false,
                }
            } else {
                true
            }
        })
        .cloned()
        .collect();
}

pub fn sort_addonfiledetails_by(
    files: &mut Vec<AddonFileDetail>,
    latest_only: bool,
) -> Vec<AddonFileDetail> {
    files.sort_by_key(|file| file.fileDate);

    if latest_only {
        let mut versions: HashSet<String> = HashSet::new();
        files.retain(|file| {
            if file.gameVersion.iter().all(|v| versions.contains(v)) {
                return false;
            } else {
                file.gameVersion.iter().for_each(|v| {
                    versions.insert(v.clone());
                    ()
                });
            }
            true
        });
    }

    return files.to_vec();
}

pub async fn search_multiple_candidates(slug: &str) -> Result<Addon, String> {
    let mut candidates: Vec<&str> = vec![slug];
    candidates.append(&mut slug.split(|c| " -+".contains(c)).collect());

    for &candidate in &candidates {
        let addons = curseforge::search(candidate)
            .await
            .or(Err(format!("failed to search '{}'", candidate)))?;
        for addon in addons {
            if addon.slug == slug {
                return Ok(addon);
            }
        }
    }

    Err("not found".to_string())
}
