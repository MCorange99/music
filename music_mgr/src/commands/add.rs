use std::str::FromStr;

use crate::{config::ConfigWrapper, downloader::Downloader, manifest::{song::Song, Manifest}, util::is_supported_host};



pub async fn add(cfg: &ConfigWrapper, manifest: &mut Manifest, downloader: &mut Downloader, url: &Option<String>, name: &Option<String>, genre: &Option<String>) -> anyhow::Result<()> {
    
    log::debug!("Genre: {genre:?}");
    log::debug!("url: {url:?}");
    log::debug!("name: {name:?}");

    let mut genres = manifest.get_genres().keys().map(|f| f.clone()).collect::<Vec<String>>();

    genres.sort();

    let genre = genre.clone().unwrap_or_else( || {
        let g = crate::prompt::prompt_with_list_or_str("Enter song genre", &genres);
        log::info!("Genre: {g}");
        g
    });


    let url = url.clone().unwrap_or_else( ||
        crate::prompt::simple_prompt("Enter song youtube url, make sure its not a playlist, (yt only for now)")
    );

    if !is_supported_host(url::Url::from_str(&url)?) {
        log::error!("Invalid or unsupported host name");
        return Ok(());
    }


    let name = name.clone().unwrap_or_else( ||
        crate::prompt::simple_prompt("Enter song name with like this: {Author} - {Song name}")
    );

    let song = Song::from_url_str(url)?;
    manifest.add_song(genre.clone(), name.clone(), song.clone());
    manifest.save(None)?;

    let should_download = crate::prompt::prompt_bool("Download song now?", Some(false));

    if should_download {
        downloader.download_song(cfg, &name, &song, &genre, manifest.get_format()).await?;
        crate::process_manager::wait_for_procs_untill(0).await?;
    }

    Ok(())
}