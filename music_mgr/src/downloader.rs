use std::{collections::HashMap, path::PathBuf, process::Stdio};

use lazy_static::lazy_static;
use log::Level;
use tokio::sync::{Mutex, RwLock};

use crate::{cli::CliArgs, manifest::Manifest};

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
    ytdlp_path: String,
    id_itr: usize,
}

impl Downloader {
    pub fn new(ytdlp_path: String) -> Self {
        Self {
            count: 0,
            ytdlp_path,
            id_itr: 0,
        }
    }

    pub async fn download_all(&mut self, manifest: &Manifest, cli: &CliArgs) -> anyhow::Result<usize> {
        let format = manifest.format()?;

        for (genre, songs) in &manifest.genres {
            for song in songs {
                self.download_song(format!("{}/{genre}/{}.{}", cli.output, song.name, &format), &format, &song.url).await?;
                self.wait_for_procs(10).await?;
            }
        }
        self.wait_for_procs(0).await?;
        Ok(self.count)
    }
    
    async fn download_song(&mut self, path: String, audio_format: &String, url: &String) -> anyhow::Result<()> {
        if PathBuf::from(&path).exists() {
            log::debug!("File {path} exists, skipping");
            return Ok(())
        }
        let mut cmd = tokio::process::Command::new(&self.ytdlp_path);
        let cmd = cmd.args([
                "-x",
                "--audio-format",
                audio_format.as_str(),
                "-o",
                path.as_str(),
                url.as_str()
            ]);

        let cmd = if log::max_level() < Level::Debug {
            cmd.stdout(Stdio::null()).stderr(Stdio::null())
        } else {cmd};

        let mut proc = cmd.spawn()?;
        let id = self.id_itr;
        
        tokio::spawn(async move {
            let id = id;
            proc.wait().await
                .expect("child process encountered an error");
            PROCESSES.lock().await.write().await.get_mut(&id).unwrap().finished = true;
        });
        
        log::info!("Downloading {path}");
        PROCESSES.lock().await.write().await.insert(id, Proc {
            url: url.clone(),
            path,
            finished: false,
        });
        self.id_itr += 1;
        Ok(())
    }

    async fn wait_for_procs(&mut self, until: usize) -> anyhow::Result<()> {
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
        #[allow(unreachable_code)] //? rust_analizer not smart enough for this
        Ok(())
    }
}