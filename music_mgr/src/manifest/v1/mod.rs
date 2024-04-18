pub mod song;

use std::{collections::HashMap, path::PathBuf, str::FromStr};
use anyhow::Result;
use serde::{Deserialize, Serialize};

use self::song::SongV1;

use super::{Format, Genre, GenreName, Manifest, Song, SongName};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestV1 {
    version: usize,
    format: String,
    genres: HashMap<GenreName, HashMap<SongName, Box<SongV1>>>
}




impl Manifest for ManifestV1 {
    fn get_format(&self) -> Result<super::Format> {
        Ok(Format::try_from(self.format.clone())?)
    }

    fn add_song(&mut self, genre: GenreName, name: SongName, song: &dyn Song) -> Option<Box<dyn Song>> {
        let song: SongV1 = song.into();
        self.get_genre_mut(genre)?
            .insert(name, Box::new(song))
    }

    fn get_song(&self, genre: GenreName, name: SongName) -> Option<&Box<dyn Song>> {
        self.get_genre(genre)?.get(&name)
    }

    fn get_song_mut(&mut self, genre: GenreName, name: SongName) -> Option<&mut Box<dyn Song>> {
        self.get_genre_mut(genre)?.get_mut(&name)
    }

    fn add_genre(&mut self, name: GenreName) {
        self.genres.insert(name, Default::default());
    }

    fn get_genre(&self, genre: GenreName) -> Option<&super::Genre> {
        unsafe {
            std::mem::transmute(self.genres.get(&genre))
        }
    }

    fn get_genre_mut(&mut self, genre: GenreName) -> Option<&mut super::Genre> {
        unsafe {
            std::mem::transmute(self.genres.get_mut(&genre))
        }
    }

    fn get_genres(&self) -> &HashMap<GenreName, super::Genre> {
        &self.genres
    }

    fn get_genres_mut(&mut self) -> &mut HashMap<GenreName, super::Genre> {
        todo!()
    }

    fn load(&mut self, p: PathBuf) {
        todo!()
    }

    fn save(&self, p: Option<&PathBuf>) {
        todo!()
    }
}