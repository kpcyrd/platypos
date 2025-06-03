use crate::args;
use crate::db::Database;
use crate::depends::Depends;
use crate::errors::*;
use crate::pkginfo;
use crate::srcinfo;
use std::fs;
use std::path::Path;
use std::sync::{Arc, mpsc};
use threadpool::ThreadPool;

#[derive(Debug, Default)]
struct PkgBase {
    db: Database,
    depends: Depends,
}

fn pkgbase(path: &Path, args: &args::Make) -> Result<Option<PkgBase>> {
    // check if the package is eligible
    if !path.is_dir() {
        return Ok(None);
    }

    if path.join("DEFUNCT").exists() {
        debug!("Skipping currently-defunct package: {path:?}");
        return Ok(None);
    }

    // read srcinfo
    let srcinfo_path = path.join(".SRCINFO");
    debug!("Reading SRCINFO: {srcinfo_path:?}");
    let srcinfo = fs::read_to_string(&srcinfo_path)
        .with_context(|| format!("Failed to read file: {srcinfo_path:?}"))?;

    // read pkginfo
    let mut pkgbase = PkgBase::default();
    for srcinfo in srcinfo::parse(&srcinfo)? {
        if ![&args.arch, "any"].iter().any(|a| *a == srcinfo.arch) {
            debug!("Skipping unsupported architecture: {srcinfo:?}");
            continue;
        }
        info!("pkg/srcinfo: {srcinfo:?}");
        let (meta, pkg) = pkginfo::load(path, &srcinfo)?;
        info!("pkg/pkginfo: {pkg:?}");
        pkgbase.db.insert(&meta, &pkg)?;
        pkgbase.depends.insert(&pkg);

        let filename = srcinfo.filename();
        let src_path = path.join(&filename);
        let output_path = args.output.join(&filename);
        if !output_path.exists() {
            fs::copy(&src_path, &output_path)
                .with_context(|| format!("Failed to copy {src_path:?} -> {output_path:?}"))?;
        }
    }

    Ok(Some(pkgbase))
}

fn start_threadpool(
    args: &Arc<args::Make>,
    tx: mpsc::Sender<Result<Option<PkgBase>>>,
) -> Result<()> {
    let pool = ThreadPool::new(num_cpus::get());
    for entry in fs::read_dir(&args.path)? {
        let entry = entry?;
        let path = entry.path();
        let args = args.clone();
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(pkgbase(&path, &args)).ok();
        });
    }
    Ok(())
}

pub fn run(args: args::Make) -> Result<()> {
    let mut db = Database::default();
    let mut depends = Depends::default();

    fs::create_dir_all(&args.output)
        .with_context(|| format!("Failed to create output directory: {:?}", args.output))?;

    // start threadpool
    let args = Arc::new(args);
    let (tx, rx) = mpsc::channel();
    start_threadpool(&args, tx)?;

    // collect results from threadpool with error handling
    let mut first_error = None;
    for result in rx {
        match result {
            Ok(Some(mut pkgbase)) => {
                db.append(&mut pkgbase.db);
                depends.append(&mut pkgbase.depends);
            }
            Ok(None) => (),
            Err(err) => {
                error!("Error from threadpool: {err:#}");
                first_error.get_or_insert(err);
            }
        }
    }
    if let Some(err) = first_error {
        return Err(err);
    }

    // generate database
    let output_path = args.output.join(format!("{}.db", args.name));
    let mut output = fs::File::create(&output_path)
        .with_context(|| format!("Failed to open output file: {:?}", output_path))?;
    db.write_into(&mut output)
        .context("Failed to write database")?;

    depends.lint();

    Ok(())
}
