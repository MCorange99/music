use crate::{config::ConfigWrapper, downloader::Downloader, manifest::{Manifest, ManifestSong}};



pub async fn add(cfg: &ConfigWrapper, manifest: &mut Manifest, downloader: &mut Downloader, url: &Option<String>, name: &Option<String>, genre: &Option<String>) -> anyhow::Result<()> {
    
    log::debug!("Genre: {genre:?}");
    log::debug!("url: {url:?}");
    log::debug!("name: {name:?}");

    let mut genres = manifest.genres.keys().map(|f| f.clone()).collect::<Vec<String>>();

    genres.sort();

    let genre = genre.clone().unwrap_or_else( || {
        let g = crate::prompt::prompt_with_list_or_str("Enter song genre", &genres);
        log::info!("Genre: {g}");
        g
    });


    let url = url.clone().unwrap_or_else( ||
        crate::prompt::simple_prompt("Enter song youtube url, make sure its not a playlist, (yt only for now)")
    );

    let name = name.clone().unwrap_or_else( ||
        crate::prompt::simple_prompt("Enter song name with like this: {Author} - {Song name}")
    );

    manifest.add_song(genre.clone(), name.clone(), url.clone())?;
    manifest.save()?;

    let should_download = crate::prompt::prompt_bool("Download song now?", Some(false));

    if should_download {
        let song = &ManifestSong {
            name,
            url,
        };

        downloader.download_song(cfg, song, &genre, &manifest.format()?).await?;
        downloader.wait_for_procs(0).await?;
    }

    Ok(())
}