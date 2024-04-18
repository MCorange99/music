use std::{collections::HashMap, path::PathBuf, process::Stdio};

use lazy_static::lazy_static;
use log::Level;
use tokio::sync::{Mutex, RwLock};

use crate::{config::ConfigWrapper, manifest::{song::{Song, SongType}, Format, Manifest}};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Proc {
    url: String,
    path: String,
    finished: bool
}

lazy_static!(
    static ref PROCESSES: Mutex<RwLock<HashMap<usize, Proc>>> = Mutex::new(RwLock::new(HashMap::new()));
);

pub struct Downloader {
    count: usize,
    id_itr: usize,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            count: 0,
            id_itr: 0,
        }
    }

    pub async fn download_all(&mut self, manifest: &Manifest, cfg: &ConfigWrapper) -> anyhow::Result<usize> {
        let format = manifest.get_format();

        for (genre, songs) in manifest.get_genres() {
            for (song_name, song) in songs {
                self.download_song(cfg, song_name, song, &genre, format).await?;
                self.wait_for_procs(10).await?;
            }
        }
        self.wait_for_procs(0).await?;
        Ok(self.count)
    }
    
    pub async fn download_song(&mut self, cfg: &ConfigWrapper, name: &String, song: &Song, genre: &String, format: &Format) -> anyhow::Result<()> {
        let dl_dir = format!("{}/{genre}", cfg.cli.output);
        let dl_file = format!("{dl_dir}/{}.{}", name, &format);

        if PathBuf::from(&dl_file).exists() {
            log::debug!("File {dl_file} exists, skipping");
            return Ok(())
        }

        log::debug!("File {dl_file} doesnt exist, downloading");
        let mut cmd = match song.get_type() {

            &SongType::Youtube => {
                log::debug!("Song {} is from yotube", song.get_url_str());
                let mut cmd = tokio::process::Command::new(&cfg.cfg.ytdlp.path);
                cmd.args([
                        "-x",
                        "--audio-format",
                        &format.to_string(),
                        "-o",
                        dl_file.as_str(),
                        song.get_url_str().as_str()
                    ]);
                cmd
            }
            SongType::Spotify => {

                let mut cmd = tokio::process::Command::new(&cfg.cfg.spotdl.path);
                cmd.args([
                    "--format",
                    &format.to_string(),
                    "--output",
                    dl_dir.as_str(),
                    song.get_url_str().as_str()
                ]);
                cmd
            }
            url => {
                log::error!("Unknown or unsupported hostname '{:?}'", url);
                return Ok(());
            }
        };
                
        if log::max_level() < Level::Debug {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
        };

        let mut proc = cmd.spawn()?;
        let id = self.id_itr;
        
        tokio::spawn(async move {
            let id = id;
            proc.wait().await
                .expect("child process encountered an error");
            PROCESSES.lock().await.write().await.get_mut(&id).unwrap().finished = true;
        });
        
        log::info!("Downloading {dl_file}");
        PROCESSES.lock().await.write().await.insert(id, Proc {
            url: song.get_url_str().clone(),
            path: dl_file,
            finished: false,
        });
        self.id_itr += 1;
        Ok(())
    }

    pub async fn wait_for_procs(&mut self, until: usize) -> anyhow::Result<()> {
        // NOTE: This looks really fucked because i dont want to deadlock the processes so i lock PROCESSES for as little as possible
        // NOTE: So its also kinda really slow
        loop {
            {
                if PROCESSES.lock().await.read().await.len() <= until {
                    return Ok(());
                }
            }

            let procs = {
                PROCESSES.lock().await.read().await.clone()
            };

            for (idx, proc) in procs {
                if proc.finished {
                    {
                        PROCESSES.lock().await.write().await.remove(&idx);
                    }
                    log::info!("Finished downloading {}", proc.path);
                    self.count += 1;
                }
            }
        }
        #[allow(unreachable_code)] //? rust_analyzer not smart enough for this
        Ok(())
    }
}