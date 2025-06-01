pub mod args;
pub mod db;
pub mod errors;
pub mod pkginfo;
pub mod srcinfo;

use crate::args::Args;
use crate::db::Database;
use crate::errors::*;
use clap::Parser;
use env_logger::Env;
use std::fs;

fn main() -> Result<()> {
    let args = Args::parse();

    let log_level = match args.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    env_logger::init_from_env(Env::default().default_filter_or(log_level));

    let mut db = Database::default();
    for entry in fs::read_dir(&args.path)? {
        let entry = entry?;
        let path = entry.path();

        // check if the package is eligible
        if !path.is_dir() {
            continue;
        }

        if path.join("DEFUNCT").exists() {
            debug!("Skipping currently-defunct package: {path:?}");
            continue;
        }

        // read srcinfo
        let srcinfo_path = path.join(".SRCINFO");
        debug!("Reading SRCINFO: {srcinfo_path:?}");
        let srcinfo = fs::read_to_string(&srcinfo_path)?;

        // read pkginfo
        for srcinfo in srcinfo::parse(&srcinfo)? {
            if ![&args.arch, "any"].iter().any(|a| *a == srcinfo.arch) {
                debug!("Skipping unsupported architecture: {srcinfo:?}");
                continue;
            }
            info!("pkg/srcinfo: {srcinfo:?}");
            let (meta, pkg) = pkginfo::load(&path, &srcinfo)?;
            info!("pkg/pkginfo: {pkg:?}");
            db.insert(&meta, &pkg)?;

            let filename = srcinfo.filename();
            let src_path = path.join(&filename);
            let output_path = args.output.join(&filename);
            if !output_path.exists() {
                fs::copy(&src_path, &output_path)
                    .with_context(|| format!("Failed to copy {src_path:?} -> {output_path:?}"))?;
            }
        }
    }

    let output_path = args.output.join(format!("{}.db", args.name));
    let mut output = fs::File::create(&output_path)
        .with_context(|| format!("Failed to open output file: {:?}", output_path))?;
    db.write_into(&mut output)
        .context("Failed to write database")?;

    Ok(())
}
