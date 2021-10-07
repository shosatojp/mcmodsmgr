use crate::Addon;
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

    for addon in addons {
        table.add_row(row![
            addon.id,
            addon.name,
            format!("{}/{}", addon.gameSlug, addon.slug)
        ]);
    }
    table.printstd();
}
