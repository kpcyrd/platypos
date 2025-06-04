use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
    /// Increase logging output (can be used multiple times)
    #[arg(short, long, global = true, action(ArgAction::Count))]
    pub verbose: u8,
    /// Output directory for generated html
    #[arg(short, long)]
    pub output: PathBuf,
    /// Path containing the pkg source files
    pub source: PathBuf,
}
