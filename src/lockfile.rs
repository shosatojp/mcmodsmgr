use prettytable::{row, Table};
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
    pub installed: Vec<LockfileEntry>,
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
}

impl Lockfile {
    pub fn new(path: &str) -> Lockfile {
        match Lockfile::load(&path) {
            Ok(content) => Lockfile {
                path: path.to_string(),
                content: content,
            },
            Err(_) => Lockfile {
                path: path.to_string(),
                content: LockfileContent {
                    version: LOCKFILE_VERSION.to_string(),
                    installed: vec![],
                },
            },
        }
    }

    pub fn load(path: &str) -> Result<LockfileContent, String> {
        let file = File::open(&path).or(Err("failed to open lockfile"))?;
        let buf_reader = BufReader::new(file);

        let content = serde_json::from_reader(buf_reader)
            .or(Err(&format!("failed to parse lockfile. remove {}", &path)))?;
        Ok(content)
    }

    pub fn save(&self) -> Result<(), String> {
        let file = File::create(&self.path).or(Err("failed to open lockfile"))?;
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, &self.content)
            .or(Err("failed to write lockfile"))?;

        Ok(())
    }

    pub fn add_lockfile_entry(&mut self, lockfileentry: LockfileEntry) {
        self.content.installed.push(lockfileentry);
        let _ = self.save();
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
