use crate::errors::*;
use crate::pkginfo;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::collections::BTreeMap;
use std::fmt::{self, Write as _};
use std::io::{self, Write};
use tar::Header;

pub enum Value<'a> {
    One(&'a str),
    Opt(Option<&'a str>),
    Many(&'a [String]),
}

impl Value<'_> {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::One(_value) => false,
            Self::Opt(value) => value.is_none(),
            Self::Many(values) => values.is_empty(),
        }
    }

    pub fn write_into<W: fmt::Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            Self::One(value) => {
                writeln!(writer, "{value}")?;
                writeln!(writer)?;
            }
            Self::Opt(value) => {
                if let Some(value) = value {
                    writeln!(writer, "{value}")?;
                    writeln!(writer)?;
                }
            }
            Self::Many(values) => {
                for value in *values {
                    writeln!(writer, "{value}")?;
                }
                writeln!(writer)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Database {
    entries: BTreeMap<String, String>,
}

impl Database {
    pub fn append(&mut self, other: &mut Self) {
        self.entries.append(&mut other.entries);
    }

    fn serialize(meta: &pkginfo::Meta, pkg: &pkginfo::Pkg) -> Result<String> {
        let csize = meta.compressed_size.to_string();
        let pairs = [
            ("FILENAME", Value::One(&meta.filename)),
            ("NAME", Value::Many(&pkg.name)),
            ("BASE", Value::Many(&pkg.base)),
            ("VERSION", Value::Many(&pkg.version)),
            ("DESC", Value::Many(&pkg.desc)),
            ("GROUPS", Value::Many(&pkg.groups)),
            ("CSIZE", Value::One(&csize)),
            ("ISIZE", Value::Many(&pkg.size)),
            ("SHA256SUM", Value::One(&meta.sha256)),
            ("PGPSIG", Value::Opt(meta.pgpsig.as_deref())),
            ("URL", Value::Many(&pkg.url)),
            ("LICENSE", Value::Many(&pkg.license)),
            ("ARCH", Value::Many(&pkg.arch)),
            ("BUILDDATE", Value::Many(&pkg.builddate)),
            ("PACKAGER", Value::Many(&pkg.packager)),
            ("REPLACES", Value::Many(&pkg.replaces)),
            ("CONFLICTS", Value::Many(&pkg.conflicts)),
            ("PROVIDES", Value::Many(&pkg.provides)),
            ("DEPENDS", Value::Many(&pkg.depends)),
            ("OPTDEPENDS", Value::Many(&pkg.optdepends)),
            ("MAKEDEPENDS", Value::Many(&pkg.makedepends)),
        ];

        let mut buf = String::new();
        for (key, value) in pairs {
            if value.is_empty() {
                continue;
            }
            writeln!(buf, "%{key}%")?;
            value.write_into(&mut buf)?;
        }

        Ok(buf)
    }

    pub fn insert(&mut self, meta: &pkginfo::Meta, pkg: &pkginfo::Pkg) -> Result<()> {
        let key = format!("{}-{}", pkg.name()?, pkg.version()?);

        let buf = Self::serialize(meta, pkg)?;
        self.entries.insert(key, buf);
        Ok(())
    }

    fn write_tar_dir<W: Write>(tar: &mut tar::Builder<GzEncoder<W>>, path: &str) -> Result<()> {
        let mut header = Header::new_ustar();
        header.set_mode(0o755);
        debug!("Adding entry to tar file: {path:?}");
        tar.append_data(&mut header, path, &mut io::empty())?;
        Ok(())
    }

    fn write_tar_data<W: Write>(
        tar: &mut tar::Builder<GzEncoder<W>>,
        path: &str,
        data: &str,
    ) -> Result<()> {
        let mut header = Header::new_ustar();
        header.set_size(data.len() as u64);
        header.set_mode(0o644);
        debug!("Adding entry to tar file: {path:?} ({} bytes)", data.len());
        tar.append_data(&mut header, path, data.as_bytes())?;
        Ok(())
    }

    pub fn write_into<W: Write>(&self, writer: &mut W) -> Result<()> {
        let gz = GzEncoder::new(writer, Compression::default());
        let mut tar = tar::Builder::new(gz);
        for (key, value) in &self.entries {
            Self::write_tar_dir(&mut tar, &format!("{key}/"))?;
            Self::write_tar_data(&mut tar, &format!("{key}/desc"), value)?;
        }
        let gz = tar.into_inner()?;
        let file = gz.finish()?;
        file.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_db() {
        let mut db = Database::default();
        db.insert(
            &pkginfo::Meta {
                filename: "util-linux-2.41-4-x86_64.pkg.tar.zst".to_string(),
                compressed_size: 3964142,
                sha256: "cf135a594b626487f917b8db56db7e030edd4f86a9d4f0790118fae12126d05e"
                    .to_string(),
                pgpsig: None,
            },
            &pkginfo::Pkg {
                name: vec!["util-linux".to_string()],
                base: vec!["util-linux".to_string()],
                version: vec!["2.41-4".to_string()],
                desc: vec!["Miscellaneous system utilities for Linux".to_string()],
                groups: vec![],
                url: vec!["https://github.com/util-linux/util-linux".to_string()],
                license: [
                    "BSD-2-Clause",
                    "BSD-3-Clause",
                    "BSD-4-Clause-UC",
                    "GPL-2.0-only",
                    "GPL-2.0-or-later",
                    "GPL-3.0-or-later",
                    "ISC",
                    "LGPL-2.1-or-later",
                    "LicenseRef-PublicDomain",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                arch: vec!["x86_64".to_string()],
                builddate: vec!["1748610944".to_string()],
                packager: vec!["Unknown Packager".to_string()],
                size: vec!["15530890".to_string()],
                replaces: vec!["rfkill".to_string(), "hardlink".to_string()],
                conflicts: vec!["rfkill".to_string(), "hardlink".to_string()],
                provides: vec!["rfkill".to_string(), "hardlink".to_string()],
                depends: [
                    "util-linux-libs=2.41",
                    "coreutils",
                    "file",
                    "libmagic.so=1-64",
                    "glibc",
                    "libcap-ng",
                    "libxcrypt",
                    "libcrypt.so=2-64",
                    "ncurses",
                    "libncursesw.so=6-64",
                    "pam",
                    "readline",
                    "shadow",
                    "systemd-libs",
                    "libsystemd.so=0-64",
                    "libudev.so=1-64",
                    "zlib",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                optdepends: vec!["words: default dictionary for look".to_string()],
                makedepends: [
                    "bash-completion",
                    "bison",
                    "flex",
                    "git",
                    "libcap-ng",
                    "libxcrypt",
                    "meson",
                    "sqlite",
                    "systemd",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
            },
        )
        .unwrap();
        assert_eq!(
            db,
            Database {
                entries: [(
                    "util-linux-2.41-4".to_string(),
                    "%FILENAME%
util-linux-2.41-4-x86_64.pkg.tar.zst

%NAME%
util-linux

%BASE%
util-linux

%VERSION%
2.41-4

%DESC%
Miscellaneous system utilities for Linux

%CSIZE%
3964142

%ISIZE%
15530890

%SHA256SUM%
cf135a594b626487f917b8db56db7e030edd4f86a9d4f0790118fae12126d05e

%URL%
https://github.com/util-linux/util-linux

%LICENSE%
BSD-2-Clause
BSD-3-Clause
BSD-4-Clause-UC
GPL-2.0-only
GPL-2.0-or-later
GPL-3.0-or-later
ISC
LGPL-2.1-or-later
LicenseRef-PublicDomain

%ARCH%
x86_64

%BUILDDATE%
1748610944

%PACKAGER%
Unknown Packager

%REPLACES%
rfkill
hardlink

%CONFLICTS%
rfkill
hardlink

%PROVIDES%
rfkill
hardlink

%DEPENDS%
util-linux-libs=2.41
coreutils
file
libmagic.so=1-64
glibc
libcap-ng
libxcrypt
libcrypt.so=2-64
ncurses
libncursesw.so=6-64
pam
readline
shadow
systemd-libs
libsystemd.so=0-64
libudev.so=1-64
zlib

%OPTDEPENDS%
words: default dictionary for look

%MAKEDEPENDS%
bash-completion
bison
flex
git
libcap-ng
libxcrypt
meson
sqlite
systemd

"
                    .to_string()
                ),]
                .into_iter()
                .collect(),
            }
        );
    }
}
