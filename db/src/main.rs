use clap::Parser;
use db::args::{Args, SubCommand};
use db::errors::*;
use db::ops;
use env_logger::Env;

fn main() -> Result<()> {
    let args = Args::parse();

    let log_level = match args.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    env_logger::init_from_env(Env::default().default_filter_or(log_level));

    match args.subcommand {
        SubCommand::Make(make) => ops::make::run(make),
    }
}
