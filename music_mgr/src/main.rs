use clap::Parser;

// TODO: Possibly use https://docs.rs/ytextract/latest/ytextract/ instead of ytdlp

use crate::cli::CliArgs;

mod cli;
mod manifest;
mod logger;
mod downloader;
mod util;
mod commands;
mod prompt;

#[tokio::main]
async fn main() {
    let mut cli_args = CliArgs::parse();
    cli_args.populate_extra();
    logger::init_logger(cli_args.debug);

    let manifest = match manifest::Manifest::from_path(&cli_args.manifest.as_std_path()) {
        Ok(m) => m,
        Err(e) => {
            log::error!("Failed to parse manifest file {}: {e}", cli_args.manifest);
            return;
        }
    };

    
    commands::command_run(&cli_args, &manifest).await;
}
