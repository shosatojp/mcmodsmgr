use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, string};

pub mod modloader_type {
    pub const forge: usize = 1;
    pub const fabric: usize = 4;
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddonFile {
    pub gameVersion: String,
    pub projectFileId: usize,
    pub projectFileName: String,
    pub fileType: usize,
    // pub gameVersionFlavor: null,
    pub gameVersionTypeId: Option<usize>,
    pub modLoader: Option<usize>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Addon {
    pub id: usize,
    pub name: String,
    pub slug: String,
    pub websiteUrl: String,
    pub downloadCount: f64,
    pub gameId: usize,
    pub summary: String,
    pub primaryCategoryId: usize,
    pub gameVersionLatestFiles: Vec<AddonFile>,
    pub gameSlug: String,
    pub modLoaders: Option<Vec<String>>,
    pub gameName: String,
    pub isAvailable: bool,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddonFileDetail {
    pub id: usize,
    pub displayName: String,
    pub fileName: String,
    pub fileDate: String,
    pub fileLength: usize,
    pub downloadUrl: String,
    pub isAvailable: bool,
    pub gameVersion: Vec<String>,
}
