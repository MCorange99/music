use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};


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

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    Download,
    Add {
        #[arg(long, short)]
        url: Option<String>,
        #[arg(long, short)]
        name: Option<String>,
        #[arg(long, short)]
        genre: Option<String>
    },
    Gui
}
