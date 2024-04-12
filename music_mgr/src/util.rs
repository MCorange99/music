
fn is_program_in_path(program: &str) -> Option<String> {
    if let Ok(path) = std::env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if std::fs::metadata(&p_str).is_ok() {
                return Some(p_str);
            }
        }
    }
    None
}



pub fn get_ytdlp_path() -> String {
    if let Some(p) = is_program_in_path("yt-dlp") {
        return p;
    }
    // TODO: Download yt-dlp to ./.bin/yt-dlp if doesnt exist
    todo!()
}

#[cfg(target_family="unix")]
pub fn isatty() -> bool {
    use std::{ffi::c_int, os::fd::AsRawFd};
    unsafe {
        let fd = std::io::stdin().as_raw_fd();
        libc::isatty(fd as c_int) == 1
    }
}

#[cfg(target_family="windows")]
pub fn isatty() -> bool {
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