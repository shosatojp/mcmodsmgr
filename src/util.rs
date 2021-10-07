use crate::api::curseforge;
use crate::api::curseforge_types::{modloader_type, AddonFileDetail};
use crate::{Addon, AddonFile};
use core::result::Result;
use std::io::Write;

pub async fn download_file(url: &str, path: &str) -> Result<(), String> {
    let mut file = std::fs::File::create(path).or(Err("failed to open file"))?;
    let content = reqwest::get(url)
        .await
        .or(Err("failed to request"))?
        .bytes()
        .await
        .or(Err("failed to get body"))?;
    file.write(&content);
    Ok(())
}

pub fn print_addons(addons: &Vec<Addon>) {
    let mut table = prettytable::Table::new();
    table.add_row(row!["Mod Id", "Display Name", "Short Name"]);
    for addon in addons {
        table.add_row(row![addon.id, addon.name, format!("{}", addon.slug)]);
    }
    table.printstd();
}

pub fn modloader_name2id(name: &str) -> Result<usize, ()> {
    match name {
        "forge" => Ok(1),
        "fabric" => Ok(4),
        _ => Err(()),
    }
}

pub fn game_version_tags_to_modloader(versions: &Vec<String>) -> Option<String> {
    let modloaders = vec!["forge", "fabric"];
    for version in versions {
        let lower = version.to_lowercase();
        if modloaders.contains(&lower.as_str()) {
            return Some(lower);
        }
    }
    None
}

pub fn print_files(files: &Vec<AddonFileDetail>) {
    let mut table = prettytable::Table::new();
    table.add_row(row!["File Id", "Version", "Mod Loader", "File Name"]);
    for file in files {
        let modLoader = game_version_tags_to_modloader(&file.gameVersion);

        table.add_row(row![
            file.id,
            file.gameVersion.join(", "),
            modLoader.unwrap_or("-".to_string()),
            file.fileName,
        ]);
    }
    table.printstd();
}

pub fn filter_addonfiles_by(
    addonfiles: &Vec<AddonFile>,
    version: Option<&str>,
    modloader: Option<&str>,
    fileid: Option<usize>,
    filename: Option<&str>,
) -> Vec<AddonFile> {
    // filter by version
    return addonfiles
        .iter()
        // filter by fileid
        .filter(|&gameVersionLatestFile| {
            if let Some(fileid) = fileid {
                gameVersionLatestFile.projectFileId == fileid
            } else {
                true
            }
        })
        // filter by filename
        .filter(|&gameVersionLatestFile| {
            if let Some(filename) = filename {
                gameVersionLatestFile.projectFileName == filename
            } else {
                true
            }
        })
        // filter by version
        .filter(|&gameVersionLatestFile| {
            if let Some(version) = version {
                gameVersionLatestFile.gameVersion == version
            } else {
                true
            }
        })
        // filter by modloader
        .filter(|&gameVersionLatestFile| {
            if let Some(modloader) = modloader {
                match gameVersionLatestFile.modLoader {
                    Some(val) => {
                        val == modloader_name2id(modloader).unwrap_or(modloader_type::forge)
                    }
                    None => false,
                }
            } else {
                true
            }
        })
        .cloned()
        .collect();
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

pub async fn search_multiple_candidates(slug: &str) -> Result<Addon, String> {
    let mut candidates: Vec<&str> = vec![slug];
    candidates.append(&mut slug.split(|c| " -+".contains(c)).collect());

    for &candidate in &candidates {
        let mut addons = curseforge::search(candidate).await.or(Err(""))?;
        for addon in addons {
            if addon.slug == slug {
                return Ok(addon);
            }
        }
    }

    Err("not found".to_string())
}
