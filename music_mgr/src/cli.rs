use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};

use crate::util::isatty;

#[derive(Debug, Parser)]
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

    #[command(subcommand)]
    pub command: Option<CliCommand>,

    #[clap(skip)]
    pub is_tty: bool
}

#[derive(Debug, Subcommand, Default)]
pub enum CliCommand {
    #[default]
    Download,
    Add {
        url: String,
        name: String,
        genre: String
    }
}

impl CliArgs {
    pub fn populate_extra(&mut self) -> &mut Self{
        self.is_tty = isatty();
        self
    }
}