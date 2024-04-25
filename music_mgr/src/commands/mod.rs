mod add;
pub mod gui;

use crate::{config::{cli::CliCommand, ConfigWrapper}, downloader::Downloader, manifest::Manifest};



pub async fn command_run(cfg: &ConfigWrapper, manifest: &mut Manifest) -> anyhow::Result<()> {
    log::info!("Is in term: {}", cfg.isatty);
    // std::fs::write("./isatty", format!("{}\n", cfg.isatty))?;

    let mut downloader = Downloader::new();
    match (&cfg.cli.command, cfg.isatty) {
        (None | Some(CliCommand::Download), true) => {
            match downloader.download_all(manifest, &cfg).await {
                Ok(count) => log::info!("Downloaded {count} songs"),
                Err(e) => {
                    log::error!("Failed to download songs: {e}");
                    return Ok(());
                }
            }
        },
        (Some(c), _) => {
            match c {
                CliCommand::Download => unreachable!(),
                CliCommand::Add { url, name, genre  } => {
                    if let Err(e) = add::add(cfg, manifest, &mut downloader, url, name, genre).await {
                        log::error!("Failed to run 'add' command: {e}");
                    }
                }
                CliCommand::Gui => {
                    gui::Gui::start(manifest.clone())?;
                },
            }
        }
        (None, false) => {
            gui::Gui::start(manifest.clone())?;
        },
    }

    Ok(())
}