use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};

use crate::util::isatty;

#[derive(Debug, Parser, Default)]
pub struct CliArgs {
    /// Show more info
    #[arg(long, short)]
    pub debug: bool,

    /// Path to manifest
    #[arg(long, short, default_value_t=Utf8PathBuf::from("./manifest.json"))]
    pub manifest: Utf8PathBuf,

    /// Output directory
    #[arg(long, short, default_value_t=Utf8PathBuf::from("./out"))]
    pub output: Utf8PathBuf,

    /// Config path
    #[arg(long, short, default_value_t=Utf8PathBuf::from("./config.json"))]
    pub config: Utf8PathBuf,

    #[command(subcommand)]
    pub command: Option<CliCommand>,

}

#[derive(Debug, Subcommand, Default)]
pub enum CliCommand {
    #[default]
    Download,
    Add {
        url: Option<String>,
        name: Option<String>,
        genre: Option<String>
    }
}
