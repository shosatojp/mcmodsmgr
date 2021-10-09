pub use crate::api::curseforge_types::Addon;
use crate::lockfile::LockfileEntry;
use crate::util;
use crate::{api::curseforge, lockfile::Lockfile};
use core::result::Result;
use std::path::Path;

pub async fn search(name: &str) -> Result<(), String> {
    // search & filter
    let addons: Vec<Addon> = curseforge::search(name).await.or(Err("failed to search"))?;

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
    slug: Option<&str>,
    version: Option<&str>,
    modloader: Option<&str>,
    fileid: Option<usize>,
    filename: Option<&str>,
    latest_only: bool,
    lockfile: &mut Lockfile,
    lockfile_ref: Option<Lockfile>,
) -> Result<(), String> {
    match lockfile_ref {
        Some(lockfile_ref) => install_with_ref(lockfile, lockfile_ref).await,
        None => {
            install_with_search(
                slug,
                version,
                modloader,
                fileid,
                filename,
                latest_only,
                lockfile,
            )
            .await
        }
    }
}

pub async fn install_with_ref(
    lockfile: &mut Lockfile,
    lockfile_ref: Lockfile,
) -> Result<(), String> {
    for entry in lockfile_ref.get_content().get_installed() {
        eprintln!("fetching file info for {}", &entry.slug);
        let file = match curseforge::get_file(entry.addonId, entry.fileId).await {
            Ok(file) => file,
            Err(err) => {
                println!("{:?}", &err);
                continue;
            }
        };
        if !Path::new(&file.fileName).exists() {
            eprintln!("downloading {} ...", file.fileName);
            util::download_file(&file.downloadUrl, &format!("{}", &file.fileName)).await?;
        } else {
            eprintln!("file already exists. skip");
            continue;
        }

        // skip if local and ref lockfile is same
        if lockfile.get_path() != lockfile_ref.get_path() {
            println!("writing to lockfile");
            lockfile
                .add_lockfile_entry(LockfileEntry {
                    registry: "curseforge.com".to_string(),
                    addonId: entry.addonId,
                    fileId: file.id,
                    fileName: file.fileName.to_string(),
                    slug: entry.slug.clone(),
                })
                .unwrap_or_else(|msg| println!("{}", msg));
        }
    }
    Ok(())
}

pub async fn install_with_search(
    slug: Option<&str>,
    version: Option<&str>,
    modloader: Option<&str>,
    fileid: Option<usize>,
    filename: Option<&str>,
    latest_only: bool,
    lockfile: &mut Lockfile,
) -> Result<(), String> {
    let slug = match slug {
        Some(slug) => slug,
        None => return Err("name is required".to_string()),
    };

    // search mod
    let target = match util::search_multiple_candidates(slug).await {
        Ok(value) => value,
        Err(_) => {
            eprintln!("mod not found");
            return Err("mod not found".to_string());
        }
    };

    // get files and filter
    let mut orig_files = curseforge::get_files(target.id)
        .await
        .or(Err("failed to get files"))?;
    orig_files = util::sort_addonfiledetails_by(&mut orig_files, latest_only);
    let files = util::filter_addonfiledetails_by(&orig_files, version, modloader, fileid, filename);

    // install
    match files.len() {
        1 => {
            let file = files.first().unwrap();
            if !Path::new(&file.fileName).exists() {
                eprintln!("downloading {} ...", file.fileName);
                util::download_file(&file.downloadUrl, &format!("{}", &file.fileName)).await?;
            } else {
                eprintln!("file already exists. skip");
                return Err("file already exists. skip".to_string());
            }

            lockfile
                .add_lockfile_entry(LockfileEntry {
                    registry: "curseforge.com".to_string(),
                    addonId: target.id,
                    fileId: file.id,
                    fileName: file.fileName.to_string(),
                    slug: target.slug,
                })
                .unwrap_or_else(|msg| println!("{}", msg));
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
    latest_only: bool,
) -> Result<(), String> {
    // search mod
    let target = match util::search_multiple_candidates(slug).await {
        Ok(value) => value,
        Err(_) => {
            eprintln!("mod not found");
            return Err("mod not found".to_string());
        }
    };

    // get files and filter
    let mut orig_files = curseforge::get_files(target.id)
        .await
        .or(Err("failed to get files"))?;
    orig_files = util::sort_addonfiledetails_by(&mut orig_files, latest_only);
    let files = util::filter_addonfiledetails_by(&orig_files, version, modloader, None, None);

    // print
    println!("{} (id:{})", target.name, target.id);
    println!();
    println!("{} downloads", target.downloadCount);
    println!("{}", target.websiteUrl);
    println!();
    println!("files:");
    util::print_files(&files);
    Ok(())
}

pub fn list(lockfile: &Lockfile) {
    lockfile.print();
    let mods_count = lockfile.get_content().get_installed().len();
    println!("{} mods installed", mods_count);
}
