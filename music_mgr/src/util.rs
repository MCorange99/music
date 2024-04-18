use std::path::PathBuf;

use crate::constants;

pub(crate) fn escape_song_name(s: String) -> String {
    s
        .replace(".", "")
        .replace("'", "")
}

pub(crate) fn is_supported_host(url: url::Url) -> bool {
    let host = url.host_str();
    if host.is_none() {
        return false;
    }
    match host.unwrap() {
        "youtube.com" | "youtu.be" |
        "open.spotify.com"  => true,
        _ => false
    }
}

pub(crate) fn is_program_in_path(program: &str) -> Option<PathBuf> {
    if let Ok(path) = std::env::var("PATH") {
        for p in path.split(constants::PATH_VAR_SEP) {
            let exec_path = PathBuf::from(p).join(program).with_extension(constants::EXEC_EXT);
            if std::fs::metadata(&exec_path).is_ok() {
                return Some(exec_path);
            }
        }
    }
    None
}

#[cfg(target_family="unix")]
pub(crate) fn isatty() -> bool {
    use std::{ffi::c_int, os::fd::AsRawFd};
    unsafe {
        let fd = std::io::stdin().as_raw_fd();
        libc::isatty(fd as c_int) == 1
    }
}

#[cfg(target_family="windows")]
pub(crate) fn isatty() -> bool {
    unsafe {
        use windows::Win32::System::Console;
        use Console::{CONSOLE_MODE, STD_OUTPUT_HANDLE};
        let Ok(handle) = Console::GetStdHandle(STD_OUTPUT_HANDLE) else {
            return false;
        }; 
        
        let mut out = CONSOLE_MODE(0);

        let ret = Console::GetConsoleMode(handle, &mut out);

        ret.is_ok()
    }
}

// pub async fn dl_to_file(url: &str, p: PathBuf) -> anyhow::Result<()> {
//     log::info!("Downloading {} -> {:?}", url, p);
//     let ytdlp_req = reqwest::get(url).await?.bytes().await?;
//     log::debug!("Downloading {:?} finished, writing to file", p);
//     let mut fd = std::fs::File::create(&p)?;
//     fd.write(&ytdlp_req)?;
//     log::debug!("Finished writing {:?}", p);
//     Ok(())
// }