use config::ConfigWrapper;


// TODO: Possibly use https://docs.rs/ytextract/latest/ytextract/ instead of ytdlp
mod manifest;
mod logger;
mod downloader;
mod util;
mod commands;
mod prompt;
mod config;
mod constants;
mod process_manager;

#[tokio::main]
async fn main() {
    let Ok(cfg) = ConfigWrapper::parse().await else {
        return;
    };

    let mut manifest = match manifest::Manifest::load_new(&cfg.cli.manifest.clone().into_std_path_buf()) {
        Ok(m) => m,
        Err(e) => {
            log::error!("Failed to parse manifest file {}: {e}", cfg.cli.manifest);
            return;
        }
    };

    
    let _ = commands::command_run(&cfg, &mut manifest).await;

    // let mut s = String::new();
    // let _ = std::io::stdin().read_line(&mut s);
}
