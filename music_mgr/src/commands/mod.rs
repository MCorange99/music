mod add;

use crate::{config::{cli::CliCommand, ConfigWrapper}, downloader::Downloader, manifest::Manifest};



pub async fn command_run(cfg: &ConfigWrapper, manifest: &mut Manifest) -> anyhow::Result<()> {
    let mut downloader = Downloader::new();
    match &cfg.cli.command {
        None | Some(CliCommand::Download) => {
            match downloader.download_all(manifest, &cfg).await {
                Ok(count) => log::info!("Downloaded {count} songs"),
                Err(e) => {
                    log::error!("Failed to download songs: {e}");
                    return Ok(());
                }
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