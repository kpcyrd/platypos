use crate::errors::*;
use crate::pkginfo;
use std::collections::BTreeSet;

#[derive(Debug, Default)]
pub struct Depends {
    available: BTreeSet<String>,
    needed: BTreeSet<String>,
}

impl Depends {
    pub fn append(&mut self, other: &mut Self) {
        self.available.append(&mut other.available);
        self.needed.append(&mut other.needed);
    }

    fn add_available(&mut self, pkg: &str) {
        let (pkg, _) = pkg.split_once('=').unwrap_or((pkg, ""));
        self.available.insert(pkg.to_string());
    }

    fn add_depends(&mut self, depends: &str) {
        let (depends, _) = depends.split_once('=').unwrap_or((depends, ""));
        let depends = depends.strip_suffix('>').unwrap_or(depends);
        self.needed.insert(depends.to_string());
    }

    pub fn insert(&mut self, pkg: &pkginfo::Pkg) {
        if let Ok(name) = pkg.name() {
            self.add_available(name);
        }
        for provides in &pkg.provides {
            self.add_available(provides);
        }
        for depends in &pkg.depends {
            self.add_depends(depends);
        }
        for depends in &pkg.makedepends {
            self.add_depends(depends);
        }
    }

    pub fn lint(&self) {
        for pkg in &self.needed {
            if !self.available.contains(pkg) {
                warn!("Missing dependency: {pkg:?}");
            }
        }
    }
}
