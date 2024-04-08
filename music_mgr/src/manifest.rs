use std::{collections::HashMap, fs::read_to_string, path::Path};

use anyhow::bail;
use serde::{Deserialize, Serialize};

const ALLOWED_FORMATS: &[&'static str] = &["m4a", "aac", "flac", "mp3", "vaw"];



type Genre = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    format: String,
    pub genres: HashMap<Genre, Vec<ManifestSong>>
}

impl Manifest {
    pub fn format(&self) -> anyhow::Result<String> {
        if !ALLOWED_FORMATS.contains(&self.format.as_str()) {
            log::error!("Unknown format, allowed formats: {}", ALLOWED_FORMATS.join(", "));
            bail!("")
        }
        Ok(self.format.clone())
    }
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSong {
    pub name: String,
    pub url: String
}


impl Manifest {
    pub fn from_string(s: String) -> anyhow::Result<Self> {
        let s = serde_json::from_str(&s)?;
        Ok(s)
    }

    pub fn from_path(p: &Path) -> anyhow::Result<Self> {
        let data = read_to_string(p)?;
        Self::from_string(data)
    }
}