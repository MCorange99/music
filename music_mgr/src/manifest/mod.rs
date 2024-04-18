// pub mod v1;

pub mod song;
use song::Song;

use std::{collections::HashMap, fmt::{Debug, Display}, path::PathBuf};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

pub type GenreName = String;
pub type SongName = String;
pub type Genre = HashMap<SongName, song::Song>;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum Format {
    #[default]
    m4a,
    aac,
    flac,
    mp3,
    vaw,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Manifest {
    #[serde(skip)]
    path: PathBuf,
    format: Format,
    genres: HashMap<GenreName, Genre>
}

#[allow(dead_code)]
impl Manifest {
    pub fn get_format(&self) -> &Format {
        &self.format
    }
    pub fn add_song(&mut self, genre: GenreName, name: SongName, song: Song) -> Option<Song> {
        self.get_genre_mut(genre)?.insert(name, song)
    }
    pub fn get_song(&self, genre: GenreName, name: &SongName) -> Option<&Song> {
        self.get_genre(genre)?.get(name)
    }
    pub fn get_song_mut(&mut self, genre: GenreName, name: &SongName) -> Option<&mut Song> {
        self.get_genre_mut(genre)?.get_mut(name)
    }
    pub fn add_genre(&mut self, name: GenreName) {
        self.genres.insert(name, Default::default());
    }
    pub fn get_genre(&self, name: GenreName) -> Option<&Genre> {
        self.genres.get(&name)
    }
    pub fn get_genre_mut(&mut self, name: GenreName) -> Option<&mut Genre> {
        self.genres.get_mut(&name)
    }
    pub fn get_genres(&self) -> &HashMap<GenreName, Genre> {
        &self.genres
    }
    pub fn get_genres_mut(&mut self) -> &mut HashMap<GenreName, Genre> {
        &mut self.genres
    }
    pub fn load(&mut self, p: Option<&PathBuf>) -> Result<()> {
        let path = p.unwrap_or(&self.path);
        log::debug!("Path: {path:?}");
        let data = std::fs::read_to_string(path)?;

        let s: Self = serde_json::from_str(data.as_str())?;
        self.genres = s.genres;
        self.format = s.format;

        Ok(())
    }
    pub fn save(&self, p: Option<&PathBuf>) -> Result<()> {
        let path = p.unwrap_or(&self.path);
        log::debug!("Path: {path:?}");
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
    pub fn load_new(p: &PathBuf) -> Result<Self> {
        let mut s = Self::default();
        log::debug!("Path: {p:?}");
        s.path = p.clone();
        s.load(Some(p))?;
        Ok(s)
    }
}




impl TryFrom<String> for Format {
    type Error = anyhow::Error;
    fn try_from(value: String) -> std::prelude::v1::Result<Self, Self::Error> {
        match value.as_str() {
            "m4a" => Ok(Self::m4a),
            "aac" => Ok(Self::aac),
            "flac" => Ok(Self::flac),
            "mp3" => Ok(Self::mp3),
            "vaw" => Ok(Self::vaw),
            v => bail!("Unknown format {v}")
        }
    }
    
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::m4a => write!(f, "m4a")?,
            Format::aac => write!(f, "aac")?,
            Format::flac => write!(f, "flac")?,
            Format::mp3 => write!(f, "mp3")?,
            Format::vaw => write!(f, "vaw")?,
        }
        Ok(())
    }
}

