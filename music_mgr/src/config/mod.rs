pub mod cli;

use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::util::{self, isatty};

use self::cli::CliArgs;

// const YTDLP_DL_URL: &'static str = "https://github.com/yt-dlp/yt-dlp/archive/refs/heads/master.zip";
// const SPOTDL_DL_URL: &'static str = "https://github.com/spotDL/spotify-downloader/archive/refs/heads/master.zip";

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
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigYtdlp {
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigSpotdl {
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
            log::info!("Config doesnt exist");
            return Self::setup_config(&cli).await;
        }

        let data = std::fs::read_to_string(&cli.config)?;
        let data: Self = serde_json::from_str(&data)?;
        Ok(data)
    }

    async fn setup_config(cli: &CliArgs) -> Result<Self> {
        let mut s = Self::default();
        let mut error = false;

        match util::is_program_in_path("yt-dlp") {
            Some(p) => {
                s.ytdlp.path = p;
            },

            None => {
                error = true;
                log::error!("could not find yt-dlp, please install it.");
                log::info!(" - With winget (Windows only) (recommended):");
                log::info!("   - Most new windows versions have winget installed, if not, instructions here: https://learn.microsoft.com/en-us/windows/package-manager/winget/#install-winget");
                log::info!("   - run `winget install yt-dlp`");
                log::info!(" - With chocolatey (Windows only):");
                log::info!("   - Make sure you have chocolatey installed - https://chocolatey.org/install");
                log::info!("   - run `choco install yt-dlp` as Admin");
                log::info!(" - With pip (from python) (Cross platform)");
                log::info!("   - Make sure you have python installed");
                log::info!("   - pip install yt-dlp");
                log::info!(" - Using your distro's package manager (Unix/BSD only) (Not recommended)")
            }
        }

        match util::is_program_in_path("spotdl") {
            Some(p) => {
                s.spotdl.path = p;
            },

            None => {
                let res = crate::prompt::prompt_bool("Spotdl is not installed but if you dont need to download music from spotify you dont need it, skip it?", None);
                if res {
                    s.spotdl.path = PathBuf::from("UNUSED");
                } else {
                    error = true;
                    log::error!("could not find spotdl, please install it. ");
                    log::info!(" - With pip (from python) (Cross platform) (recommended)");
                    log::info!("   - Make sure you have python installed - https://www.python.org/downloads/");
                    log::info!("   - pip install spotdl");
                }
            }
        }

        match util::is_program_in_path("ffmpeg") {
            Some(_) => (),

            None => {
                error = true;
                log::error!("could not find ffmpeg, please install it.");
                log::info!(" - With winget (Windows only) (recommended):");
                log::info!("   - Most new windows versions have winget installed, if not, instructions here: https://learn.microsoft.com/en-us/windows/package-manager/winget/#install-winget");
                log::info!("   - run `winget install --id=Gyan.FFmpeg -e`");
                log::info!(" - With chocolatey (Windows only):");
                log::info!("   - Make sure you have chocolatey installed - https://chocolatey.org/install");
                log::info!("   - run `choco install ffmpeg` as Admin");
            }
        }

        if !error {
            s.save(cli.config.clone().into_std_path_buf())?;
        }

        Ok(s)
    }

    fn save(&self, path: PathBuf) -> anyhow::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
}
