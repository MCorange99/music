use std::str::FromStr;

use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::manifest::{Song, SongType};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongV1 {
    url: String,
    typ: SongType
}

impl SongV1 {
    pub fn from_str(url: String) -> Result<Self> {
        Self::from_url(url::Url::from_str(&url)?)
    }

    pub fn from_url(url: url::Url) -> Result<Self> {
        let typ = SongType::try_from(url.clone())?;
        Ok(Self {
            url: url.to_string(),
            typ,
        })
    }
}

impl From<&dyn Song> for SongV1 {
    fn from(value: &dyn Song) -> Self {
        Self {
            url: value.get_url_str().clone(),
            typ: value.get_type().clone(),
        }
    }
}

impl Song for SongV1 {
    fn get_url(&self) -> anyhow::Result<url::Url> {
        Ok(url::Url::from_str(&self.url)?)
    }

    fn get_url_str(&self) -> &String {
        &self.url
    }

    fn get_url_str_mut(&mut self) -> &mut String {
        &mut self.url
    }

    fn get_type(&self) -> &crate::manifest::SongType {
        &self.typ
    }

    fn get_type_mut(&mut self) -> &mut crate::manifest::SongType {
        &mut self.typ
    }
}