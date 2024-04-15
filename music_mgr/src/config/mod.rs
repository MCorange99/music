pub mod cli;

use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::util::{self, dl_to_file, isatty};

use self::cli::CliArgs;

const YTDLP_DL_URL: &'static str = "https://github.com/yt-dlp/yt-dlp/archive/refs/heads/master.zip";
const SPOTDL_DL_URL: &'static str = "https://github.com/spotDL/spotify-downloader/archive/refs/heads/master.zip";

#[derive(Debug, Default)]
pub struct ConfigWrapper {
    pub cfg: Config,
    pub cli: cli::CliArgs,
    pub isatty: bool
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub ytdlp: ConfigYtdlp,
    pub spotdl: ConfigSpotdl,
    pub python: ConfigPython,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigYtdlp {
    pub path: PathBuf,
    pub is_python: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigSpotdl {
    pub path: PathBuf,
    pub is_python: bool
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigPython {
    pub path: PathBuf,
}


impl ConfigWrapper {
    pub async fn parse() -> Result<Self> {
        let mut s = Self::default();
        s.cli = cli::CliArgs::parse();
        crate::logger::init_logger(s.cli.debug);
        s.cfg = Config::parse(&s.cli).await?;
        s.isatty = isatty();
        Ok(s)
    }
}

impl Config {
    pub async fn parse(cli: &CliArgs) -> Result<Self> {
        if !cli.config.exists() {
            return Self::setup_config(&cli).await;
        }

        let data = std::fs::read_to_string(&cli.config)?;
        let data: Self = serde_json::from_str(&data)?;
        Ok(data)
    }

    async fn setup_config(cli: &CliArgs) -> Result<Self> {
        let mut s = Self::default();

        let bin_dir = cli.output.clone().into_std_path_buf().join(".bin/");
        let mut python_needed = false;

        match util::is_program_in_path("yt-dlp") {
            Some(p) => {
                s.ytdlp.path = p;
                s.ytdlp.is_python = false;
            },

            None => {
                python_needed = true;
                s.ytdlp.is_python = true;
                s.ytdlp.path = bin_dir.join("ytdlp");
                dl_to_file(YTDLP_DL_URL, s.ytdlp.path.join("ytdlp.zip")).await?;
                zip_extensions::zip_extract(&s.ytdlp.path.join("ytdlp.zip"), &s.ytdlp.path)?;
            }
        }

        match util::is_program_in_path("spotdl") {
            Some(p) => {
                s.spotdl.path = p;
                s.spotdl.is_python = false;
            },

            None => {
                python_needed = true;
                s.spotdl.is_python = true;
                s.spotdl.path = bin_dir.join("ytdlp");
                dl_to_file(SPOTDL_DL_URL, s.spotdl.path.join("spotdl.zip")).await?;
                zip_extensions::zip_extract(&s.spotdl.path.join("spotdl.zip"), &s.ytdlp.path)?;
            }
        }


        let python_paths = &[
            util::is_program_in_path("python"),
            util::is_program_in_path("python3")
        ];

        if python_needed {
            let mut found = false;
            for p in python_paths {
                match p {
                    Some(p) => {
                        s.python.path = p.clone();
                        found = true;
                        break
                    }
                    None => {
                    }
                }
            }

            if !found {
                panic!("Python needs to be installed for this to work, or install ytdlp and spotdl manually, (dont forget to delete the config file after doing so)");
            }
        }

        Ok(s)
    }
}
