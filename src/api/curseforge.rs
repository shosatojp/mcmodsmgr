const BASE_URL: &'static str = "https://addons-ecs.forgesvc.net/api/v2";
const MINECRAFT_GAMEID: usize = 432;

use crate::api::curseforge_types::AddonFileDetail;
pub use crate::api::curseforge_types::{Addon, AddonFile};

pub async fn search(query: &str) -> Result<Vec<Addon>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/addon/search?gameId={}&searchFilter={}",
        BASE_URL, MINECRAFT_GAMEID, query
    );
    let body = reqwest::get(url).await?.text().await?;

    let addons: Vec<Addon> = serde_json::from_str(&body)?;

    Ok(addons)
}

pub async fn get_files(addon_id: usize) -> Result<Vec<AddonFileDetail>, Box<dyn std::error::Error>> {
    let url = format!("{}/addon/{}/files", BASE_URL, addon_id);
    let body = reqwest::get(url).await?.text().await?;
    
    let files: Vec<AddonFileDetail> = serde_json::from_str(&body)?;

    Ok(files)
}
