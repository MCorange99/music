
#[cfg(target_family="windows")]
mod constants {
    pub const PATH_VAR_SEP: &'static str = ";";
    pub const EXEC_EXT: &'static str = "exe";
}

#[cfg(target_family="unix")]
mod constants {
    pub const PATH_VAR_SEP: &'static str = ":";
    pub const EXEC_EXT: &'static str = "";
}


pub use constants::*;