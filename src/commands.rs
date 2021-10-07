use crate::api::curseforge;
pub use crate::api::curseforge_types::Addon;
use crate::util;

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
    slug: &str,
    version: Option<&str>,
    modloader: Option<&str>,
    fileid: Option<usize>,
    filename: Option<&str>,
    latest_only: bool,
) -> Result<(), String> {
    // search
    let target = match util::search_multiple_candidates(slug).await {
        Ok(value) => value,
        Err(_) => {
            eprintln!("mod not found");
            return Err("mod not found".to_string());
        }
    };

    // filter by...
    let mut orig_files = curseforge::get_files(target.id)
        .await
        .or(Err("failed to get files"))?;
    orig_files = util::sort_addonfiledetails_by(&mut orig_files, latest_only);
    let files = util::filter_addonfiledetails_by(&orig_files, version, modloader, fileid, filename);

    match files.len() {
        1 => {
            let file = files.first().unwrap();
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
    latest_only: bool,
) -> Result<(), String> {
    // search
    let target = match util::search_multiple_candidates(slug).await {
        Ok(value) => value,
        Err(_) => {
            eprintln!("mod not found");
            return Err("mod not found".to_string());
        }
    };

    // filter by...
    let mut orig_files = curseforge::get_files(target.id)
        .await
        .or(Err("failed to get files"))?;
    orig_files = util::sort_addonfiledetails_by(&mut orig_files, latest_only);
    let files = util::filter_addonfiledetails_by(&orig_files, version, modloader, None, None);

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