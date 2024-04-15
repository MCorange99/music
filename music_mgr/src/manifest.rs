use std::{collections::HashMap, fs::read_to_string, path::{Path, PathBuf}};

use anyhow::bail;
use serde::{Deserialize, Serialize};

const ALLOWED_FORMATS: &[&'static str] = &["m4a", "aac", "flac", "mp3", "vaw"];



type Genre = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(skip)]
    path: PathBuf,
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
    fn from_string(s: String) -> anyhow::Result<Self> {
        let s = serde_json::from_str(&s)?;
        Ok(s)
    }

    pub fn from_path(p: &Path) -> anyhow::Result<Self> {
        let data = read_to_string(p)?;
        let mut s = Self::from_string(data)?;
        s.path = p.to_path_buf();
        Ok(s)
    }

    pub fn add_song(&mut self, genre: String, name: String, url: String) -> anyhow::Result<()> {
        let Some(genre_ref) = self.genres.get_mut(&genre) else {
            log::error!("Invalid genre '{}'", genre);
            bail!("Invalid genre")
        };

        genre_ref.push(ManifestSong {
            name,
            url,
        });

        Ok(())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(&self.path, data)?;
        Ok(())
    }
}