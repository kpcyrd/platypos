pub mod args;
pub mod assets;
pub mod errors;
pub mod html;

use crate::args::Args;
use crate::errors::*;
use crate::html::Html;
use clap::Parser;
use db::srcinfo;
use env_logger::Env;
use std::fs;
use std::path::Path;

fn pkgbase(path: &Path) -> Result<Vec<srcinfo::Pkg>> {
    // check if the package is eligible
    if !path.is_dir() {
        return Ok(Default::default());
    }

    if path.join("DEFUNCT").exists() {
        debug!("Skipping currently-defunct package: {path:?}");
        return Ok(Default::default());
    }

    // read srcinfo
    let srcinfo_path = path.join(".SRCINFO");
    debug!("Reading SRCINFO: {srcinfo_path:?}");
    let srcinfo = fs::read_to_string(&srcinfo_path)
        .with_context(|| format!("Failed to read file: {srcinfo_path:?}"))?;

    srcinfo::parse(&srcinfo)
}

fn write(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create parent directory: {:?}", parent))?;
    }
    info!("Writing file: {:?}", path);
    fs::write(path, content).with_context(|| format!("Failed to write file: {:?}", path))?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let log_level = match args.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    env_logger::init_from_env(Env::default().default_filter_or(log_level));

    fs::create_dir_all(&args.output)
        .with_context(|| format!("Failed to create output directory: {:?}", args.output))?;

    let mut pkgs = Vec::new();
    for entry in fs::read_dir(args.source).context("Failed to read source directory")? {
        let entry = entry?;
        let path = entry.path();
        let srcinfo =
            pkgbase(&path).with_context(|| format!("Failed to process pkgbase: {:?}", path))?;
        pkgs.extend(srcinfo);
    }

    let html = Html::new().with_context(|| "Failed to initialize HTML renderer")?;

    write(&args.output.join("index.html"), &html.index(&pkgs)?)
        .with_context(|| "Failed to write style.css")?;
    write(&args.output.join("assets/style.css"), &html.style()?)
        .with_context(|| "Failed to write style.css")?;
    for pkg in pkgs {
        write(
            &args
                .output
                .join("pkgs/main")
                .join(&pkg.arch)
                .join(&pkg.pkgname)
                .join("index.html"),
            &html.pkg(&pkg)?,
        )
        .with_context(|| "Failed to write style.css")?;
    }

    Ok(())
}
