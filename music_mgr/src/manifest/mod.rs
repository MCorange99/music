// pub mod v1;

use std::{collections::HashMap, fmt::{Debug, Display}, path::{Path, PathBuf}, str::FromStr};

use anyhow::{bail, Result};
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
        let data = std::fs::read_to_string(p)?;
        let mut s = Self::from_string(data)?;
        s.path = p.to_path_buf();
        Ok(s)
    }

    pub fn add_song(&mut self, genre: String, name: String, url: String) -> anyhow::Result<()> {

        if !self.genres.contains_key(&genre) {
            self.genres.insert(genre.clone(), Vec::new());
        }

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
// pub type GenreName = String;
// pub type SongName = String;
// pub type Genre = HashMap<SongName, Box<dyn Song>>;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub enum SongType {
//     Youtube,
//     Spotify,
//     Soundcloud,
// }

// impl TryFrom<url::Url> for SongType {
//     type Error = anyhow::Error;

//     fn try_from(url: url::Url) -> std::prelude::v1::Result<Self, Self::Error> {
//         let Some(host) = url.host_str() else {
//             bail!("{url} does not have a host");
//         };

//         match host {
//             "youtube.com" | "youtu.be"  => Ok(Self::Youtube),
//             "open.spotify.com"  => Ok(Self::Spotify),
//             "SOUNDCLOUD" => Ok(Self::Soundcloud), // TODO: Fix this
//             _ => bail!("Unknwon host {url}")
//         }
//     }
// }


// #[allow(non_camel_case_types)]
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub enum Format {
//     m4a,
//     aac,
//     flac,
//     mp3,
//     vaw,
// }

// impl TryFrom<String> for Format {
//     type Error = anyhow::Error;
//     fn try_from(value: String) -> std::prelude::v1::Result<Self, Self::Error> {
//         match value.as_str() {
//             "m4a" => Ok(Self::m4a),
//             "aac" => Ok(Self::aac),
//             "flac" => Ok(Self::flac),
//             "mp3" => Ok(Self::mp3),
//             "vaw" => Ok(Self::vaw),
//             v => bail!("Unknown format {v}")
//         }
//     }
    
// }

// impl Display for Format {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Format::m4a => write!(f, "m4a")?,
//             Format::aac => write!(f, "aac")?,
//             Format::flac => write!(f, "flac")?,
//             Format::mp3 => write!(f, "mp3")?,
//             Format::vaw => write!(f, "vaw")?,
//         }
//         Ok(())
//     }
// }
// pub trait Song: Debug +  serde_traitobject::Serialize + serde_traitobject::Deserialize{
//     fn get_url(&self) -> Result<url::Url>;
//     fn get_url_str(&self) -> &String;
//     fn get_url_str_mut(&mut self) -> &mut String;
//     fn get_type(&self) -> &SongType;
//     fn get_type_mut(&mut self) -> &mut SongType;
// }


// pub trait Manifest: Debug + Clone + serde_traitobject::Serialize + serde_traitobject::Deserialize{
//     fn get_format(&self) -> Result<Format>;
//     fn add_song(&mut self, genre: GenreName, name: SongName, song: &dyn Song) -> Option<Box<dyn Song>>;
//     fn get_song(&self, genre: GenreName, name: SongName) -> Option<&Box<dyn Song>>;
//     fn get_song_mut(&mut self, genre: GenreName, name: SongName) -> Option<&mut Box<dyn Song>>;
//     fn add_genre(&mut self, genre: GenreName);
//     fn get_genre(&self, genre: GenreName) -> Option<&Genre>;
//     fn get_genre_mut(&mut self, genre: GenreName) -> Option<&mut Genre>;
//     fn get_genres(&self) -> &HashMap<GenreName, Genre>;
//     fn get_genres_mut(&mut self) -> &mut HashMap<GenreName, Genre>;
//     fn load(&mut self, p: PathBuf);
//     fn save(&self, p: Option<&PathBuf>);
// }