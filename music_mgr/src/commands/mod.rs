mod add;

use crate::{config::{cli::CliCommand, ConfigWrapper}, downloader::Downloader, manifest::Manifest};



pub async fn command_run(cfg: &ConfigWrapper, manifest: &mut Manifest) -> anyhow::Result<()> {
    let mut downloader = Downloader::new(cfg.cfg.ytdlp.path.clone());
    match &cfg.cli.command {
        None | Some(CliCommand::Download) => {
            if let Ok(count) = downloader.download_all(manifest, &cfg).await {
                log::info!("Downloaded {count} songs");
            } else {
                log::error!("Failed to download songs");
                return Ok(());
            }
        },
        Some(c) => {
            match c {
                CliCommand::Download => unreachable!(),
                CliCommand::Add { url, name, genre  } => add::add(cfg, manifest, &mut downloader, url, name, genre).await?,
            }
        }
    }

    Ok(())
}