use log::LevelFilter;


pub fn init_logger(debug: bool) {
    let level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(level)
        .init();
}