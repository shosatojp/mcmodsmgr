use core::result::Result;
use prettytable::{row, Table};
use regex::{self, Regex};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

const LOCKFILE_VERSION: &str = "1";

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LockfileContent {
    pub version: String,
    installed: Vec<LockfileEntry>,
}

impl LockfileContent {
    pub fn get_installed(&self) -> &Vec<LockfileEntry> {
        &self.installed
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LockfileEntry {
    pub registry: String,
    pub addonId: usize,
    pub fileId: usize,
    pub fileName: String,
    pub slug: String,
    // pub gameVersion: String,
    // pub modLoader: String,
}

pub struct Lockfile {
    content: LockfileContent,
    path: String,
    is_url_regex: Regex,
}

impl Lockfile {
    pub async fn new(path: &str) -> Result<Lockfile, String> {
        let is_url = regex::Regex::new("^https?://.*").unwrap();

        // normarize path
        let (_content, path) = if is_url.is_match(path) {
            (Lockfile::load_http(path).await, path.to_string())
        } else {
            let _path = std::path::PathBuf::from(path);
            let mut abs_path = std::env::current_dir().or(Err(""))?;
            abs_path.push(_path);

            let _abs_path = match abs_path.to_str() {
                Some(p) => p,
                None => return Err("".to_string()),
            };
            (Lockfile::load(path), _abs_path.to_string())
        };

        match _content {
            Ok(content) => Ok(Lockfile {
                path: path.to_string(),
                content: content,
                is_url_regex: is_url,
            }),
            Err(_) => Ok(Lockfile {
                path: path.to_string(),
                content: LockfileContent {
                    version: LOCKFILE_VERSION.to_string(),
                    installed: vec![],
                },
                is_url_regex: is_url,
            }),
        }
    }

    pub async fn load_http(url: &str) -> Result<LockfileContent, String> {
        let text = reqwest::get(url)
            .await
            .or(Err("failed to fetch"))?
            .text()
            .await
            .or(Err("failed to get text"))?;
        let content = serde_json::from_str(text.as_str()).or(Err("failed to parse json"))?;
        Ok(content)
    }

    pub fn load(path: &str) -> Result<LockfileContent, String> {
        let file = File::open(&path).or(Err("failed to open lockfile"))?;
        let buf_reader = BufReader::new(file);

        let content = serde_json::from_reader(buf_reader)
            .or(Err(&format!("failed to parse lockfile. remove {}", &path)))?;
        Ok(content)
    }

    pub fn save(&self) -> Result<(), String> {
        if !self.is_local_file() {
            return Err("cannot write to remote file".to_string());
        }

        let file = File::create(&self.path).or(Err("failed to open lockfile"))?;
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, &self.content)
            .or(Err("failed to write lockfile"))?;

        Ok(())
    }

    pub fn add_lockfile_entry(&mut self, lockfileentry: LockfileEntry) -> Result<(), String> {
        self.content.installed.push(lockfileentry);
        self.save()
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_content(&self) -> &LockfileContent {
        &self.content
    }

    fn is_local_file(&self) -> bool {
        !self.is_url_regex.is_match(&self.path)
    }

    pub fn print(&self) {
        let mut table = Table::new();
        for entry in &self.content.installed {
            table.add_row(row![
                entry.registry,
                entry.addonId,
                entry.fileId,
                entry.fileName,
                entry.slug,
                // entry.gameVersion,
                // entry.modLoader
            ]);
        }
        table.printstd();
        ()
    }
}
