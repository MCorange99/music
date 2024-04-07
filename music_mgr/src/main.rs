use clap::Parser;

use crate::cli::{CliArgs, CliCommand};

mod cli;

fn main() {
    let cli_args = CliArgs::parse();

    match cli_args.command {
        None => {
            // TODO: Download
        },
        Some(c) => {
            match c {
                CliCommand::Download => {
                    // TODO: Download
                },
            }
        },
    }

    println!("Hello, world!");
}
