
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