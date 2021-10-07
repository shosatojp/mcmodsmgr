use crate::api::curseforge_types::modloader_type;
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

pub fn print_files(files: &Vec<AddonFile>) {
    let mut table = prettytable::Table::new();
    table.add_row(row!["File Id", "Version", "Mod Loader", "File Name"]);
    for file in files {
        let modLoader = match file.modLoader {
            Some(1) => "forge",
            Some(4) => "fabric",
            Some(_) => "forge?",
            None => "-",
        };
        table.add_row(row![
            file.projectFileId,
            file.gameVersion,
            modLoader,
            file.projectFileName,
        ]);
    }
    table.printstd();
}

pub fn filter_addonfiles_by(
    addonfiles: &Vec<AddonFile>,
    version: Option<&str>,
    modloader: Option<&str>,
) -> Vec<AddonFile> {
    // filter by version
    return addonfiles
        .iter()
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
