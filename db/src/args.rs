use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
    /// Increase logging output (can be used multiple times)
    #[arg(short, long, global = true, action(ArgAction::Count))]
    pub verbose: u8,
    /// The path to write the database to
    #[arg(short = 'o', long)]
    pub output: PathBuf,
    /// The architecture to include
    #[arg(short = 'A', long)]
    pub arch: String,
    /// The name of the pacman repo
    #[arg(short = 'n', long)]
    pub name: String,
    /// The directory to import from
    pub path: PathBuf,
}
