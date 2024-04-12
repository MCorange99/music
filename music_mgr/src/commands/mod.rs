use crate::{cli::{CliArgs, CliCommand}, downloader::Downloader, manifest::Manifest, util};


pub async fn command_run(cli: &CliArgs, manifest: &Manifest) {
    let mut downloader = Downloader::new(util::get_ytdlp_path());
    match &cli.command {
        None | Some(CliCommand::Download) => {
            if let Ok(count) = downloader.download_all(manifest, &cli).await {
                log::info!("Downloaded {count} songs");
            } else {
                log::error!("Failed to download songs");
                return;
            }
        },
        Some(c) => {
            match c {
                CliCommand::Download => unreachable!(),
                CliCommand::Add { .. } => todo!(),
            }
        }
    }
}