use crate::{config::ConfigWrapper, downloader::Downloader, manifest::{Manifest, ManifestSong}};



pub async fn add(cfg: &ConfigWrapper, manifest: &mut Manifest, downloader: &mut Downloader, url: &Option<String>, name: &Option<String>, genre: &Option<String>) -> anyhow::Result<()> {
    
    let genres = manifest.genres.keys().map(|f| f.clone()).collect::<Vec<String>>();

    let genre = genre.clone().unwrap_or(
        crate::prompt::prompt_with_list_or_str("Enter song genre", &genres)
    );

    log::debug!("Genre: {genre}");

    let url = url.clone().unwrap_or(
        crate::prompt::simple_prompt("Enter song youtube url, make sure its not a playlist, (yt only for now)")
    );

    let name = name.clone().unwrap_or(
        crate::prompt::simple_prompt("Enter song name with like this: {Author} - {Song name}")
    );

    manifest.add_song(genre.clone(), name.clone(), url.clone())?;
    manifest.save()?;

    let should_download = crate::prompt::prompt_bool("Download song now?", Some(true));

    if should_download {
        let song = &ManifestSong {
            name,
            url,
        };

        downloader.download_song(cfg, song, &genre, &manifest.format()?).await?;
    }

    Ok(())
}