use clap::Parser;

use crate::{cli::{CliArgs, CliCommand}, downloader::Downloader};

mod cli;
mod manifest;
mod logger;
mod downloader;
mod util;

#[tokio::main]
async fn main() {
    let cli_args = CliArgs::parse();
    logger::init_logger(cli_args.debug);
    let manifest = match manifest::Manifest::from_path(&cli_args.manifest.as_std_path()) {
        Ok(m) => m,
        Err(e) => {
            log::error!("Failed to parse manifest file {}: {e}", cli_args.manifest);
            return;
        }
    };

    let mut downloader = Downloader::new(util::get_ytdlp_path());

    match cli_args.command {
        None | Some(CliCommand::Download) => {
            if let Ok(count) = downloader.download_all(manifest, &cli_args).await {
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
