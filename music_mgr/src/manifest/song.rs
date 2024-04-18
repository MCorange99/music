use std::str::FromStr;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SongType {
    Youtube,
    Spotify,
    Soundcloud,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Song {
    url: String,
    typ: SongType
}

#[allow(dead_code)]
impl Song {
    pub fn from_url_str(url: String) -> Result<Self> {
        Self::from_url(url::Url::from_str(url.as_str())?)
    }

    pub fn from_url(url: url::Url) -> Result<Self> {
        Ok(Self {
            url: url.to_string(),
            typ: url.try_into()?,
        })
    }
    pub fn get_url(&self) -> Result<url::Url> {
        Ok(url::Url::from_str(&self.url)?)
    }
    pub fn get_url_str(&self) -> &String {
        &self.url
    }
    pub fn get_url_str_mut(&mut self) -> &mut String {
        &mut self.url
    }
    pub fn get_type(&self) -> &SongType {
        &self.typ
    }
    pub fn get_type_mut(&mut self) -> &mut SongType {
        &mut self.typ
    }
}



impl TryFrom<url::Url> for SongType {
    type Error = anyhow::Error;

    fn try_from(url: url::Url) -> std::prelude::v1::Result<Self, Self::Error> {
        let Some(host) = url.host_str() else {
            bail!("{url} does not have a host");
        };

        match host {
            "youtube.com" | "youtu.be" | "www.youtube.com" => Ok(Self::Youtube),
            "open.spotify.com"  => Ok(Self::Spotify),
            "SOUNDCLOUD" => Ok(Self::Soundcloud), // TODO: Fix this
            _ => bail!("Unknwon host {url}")
        }
    }
}
