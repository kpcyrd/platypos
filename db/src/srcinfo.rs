use crate::errors::*;
use std::borrow::Cow;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pkg {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: String,
    pub epoch: Option<String>,
    pub arch: String,
}

impl Pkg {
    pub fn filename(&self) -> String {
        let version = if let Some(epoch) = &self.epoch {
            Cow::Owned(format!("{}:{}", epoch, self.pkgver))
        } else {
            Cow::Borrowed(&self.pkgver)
        };
        let filename = format!(
            "{}-{}-{}-{}.pkg.tar.zst",
            self.pkgname, version, self.pkgrel, self.arch
        );
        filename
    }
}

pub fn parse(srcinfo: &str) -> Result<Vec<Pkg>> {
    let mut pkg = Pkg::default();
    let mut pkgbase = Pkg::default();
    let mut pkgs = Vec::new();

    let mut lines = srcinfo.lines();
    loop {
        let line = match lines.next() {
            Some("") => {
                if pkg.pkgname.is_empty() {
                    pkgbase = pkg;
                } else {
                    pkgs.push(pkg);
                }
                pkg = pkgbase.clone();
                continue;
            }
            None => {
                if !pkg.pkgname.is_empty() {
                    pkgs.push(pkg);
                }
                break;
            }
            Some(line) => line,
        };
        if let Some(value) = line.strip_prefix("pkgname = ") {
            pkg.pkgname = value.to_string();
        } else if let Some(value) = line.strip_prefix("\tpkgver = ") {
            pkg.pkgver = value.to_string();
        } else if let Some(value) = line.strip_prefix("\tpkgrel = ") {
            pkg.pkgrel = value.to_string();
        } else if let Some(value) = line.strip_prefix("\tepoch = ") {
            pkg.epoch = Some(value.to_string());
        } else if let Some(value) = line.strip_prefix("\tarch = ") {
            pkg.arch = value.to_string();
        }

        // dbg!(line);
    }

    Ok(pkgs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let srcinfo = "pkgbase = coreutils-uutils\n\tpkgdesc = Cross-platform Rust rewrite of the GNU coreutils\n\tpkgver = 0.1.0\n\tpkgrel = 1\n\turl = https://uutils.github.io/\n\tarch = x86_64\n\tlicense = MIT\n\tmakedepends = cargo\n\tmakedepends = clang\n\tmakedepends = python-sphinx\n\tmakedepends = rust\n\tdepends = gcc-libs\n\tdepends = glibc\n\tdepends = oniguruma\n\tprovides = coreutils\n\tconflicts = coreutils\n\treplaces = coreutils\n\toptions = !lto\n\tsource = coreutils-uutils-0.1.0.tar.gz::https://github.com/uutils/coreutils/archive/0.1.0.tar.gz\n\tsource = disable_selinux.patch\n\tsha512sums = e9b3941573b2267e6a4b17c49a64d71234e4cbf067f1180fe14bfe48012c9d400741067f6231855a6132f93138eb4bbcb1e970e98c5f937f8d3a5e5bc019f03c\n\tsha512sums = c1585671d2e0fe34a92746eae291f1be85e65c7ccfe2462cba5d324b8def76296f71b6d15738ecb2683453886de9c0650a0b9a609cad5f03b6ded89e5a91911c\n\npkgname = coreutils-uutils\n";
        let pkgs = parse(srcinfo).unwrap();
        assert_eq!(
            pkgs,
            vec![Pkg {
                pkgname: "coreutils-uutils".to_string(),
                pkgver: "0.1.0".to_string(),
                pkgrel: "1".to_string(),
                epoch: None,
                arch: "x86_64".to_string(),
            }]
        );
    }

    #[test]
    fn test_parse_advanced() {
        let srcinfo = "pkgbase = systemd\n\tpkgver = 257.6\n\tpkgrel = 1\n\turl = https://www.github.com/systemd/systemd\n\tarch = x86_64\n\tlicense = LGPL-2.1-or-later\n\tmakedepends = acl\n\tmakedepends = cryptsetup\n\tmakedepends = docbook-xsl\n\tmakedepends = gperf\n\tmakedepends = lz4\n\tmakedepends = xz\n\tmakedepends = pam\n\tmakedepends = libelf\n\tmakedepends = intltool\n\tmakedepends = iptables\n\tmakedepends = kmod\n\tmakedepends = libarchive\n\tmakedepends = libcap\n\tmakedepends = libidn2\n\tmakedepends = libgcrypt\n\tmakedepends = libmicrohttpd\n\tmakedepends = libxcrypt\n\tmakedepends = libxslt\n\tmakedepends = util-linux\n\tmakedepends = linux-api-headers\n\tmakedepends = python-jinja\n\tmakedepends = python-lxml\n\tmakedepends = quota-tools\n\tmakedepends = shadow\n\tmakedepends = git\n\tmakedepends = meson\n\tmakedepends = libseccomp\n\tmakedepends = pcre2\n\tmakedepends = audit\n\tmakedepends = kexec-tools\n\tmakedepends = libxkbcommon\n\tmakedepends = bash-completion\n\tmakedepends = p11-kit\n\tmakedepends = systemd\n\tmakedepends = libfido2\n\tmakedepends = tpm2-tss\n\tmakedepends = rsync\n\tmakedepends = bpf\n\tmakedepends = libbpf\n\tmakedepends = clang\n\tmakedepends = llvm\n\tmakedepends = curl\n\tmakedepends = gnutls\n\tmakedepends = python-pyelftools\n\tmakedepends = libpwquality\n\tmakedepends = qrencode\n\tmakedepends = lib32-gcc-libs\n\tmakedepends = python-pefile\n\tmakedepends = linux-headers\n\tconflicts = mkinitcpio<38-1\n\tsource = git+https://github.com/systemd/systemd#tag=v257.6?signed\n\tsource = 0001-Use-Arch-Linux-device-access-groups.patch\n\tsource = arch.conf\n\tsource = loader.conf\n\tsource = splash-arch.bmp\n\tsource = systemd-user.pam\n\tsource = systemd-hook\n\tsource = 20-systemd-sysusers.hook\n\tsource = 30-systemd-binfmt.hook\n\tsource = 30-systemd-catalog.hook\n\tsource = 30-systemd-daemon-reload-system.hook\n\tsource = 30-systemd-daemon-reload-user.hook\n\tsource = 30-systemd-hwdb.hook\n\tsource = 30-systemd-restart-marked.hook\n\tsource = 30-systemd-sysctl.hook\n\tsource = 30-systemd-tmpfiles.hook\n\tsource = 30-systemd-udev-reload.hook\n\tsource = 30-systemd-update.hook\n\tvalidpgpkeys = 63CDA1E5D3FC22B998D20DD6327F26951A015CC4\n\tvalidpgpkeys = A9EA9081724FFAE0484C35A1A81CEA22BC8C7E2E\n\tvalidpgpkeys = 9A774DB5DB996C154EBBFBFDA0099A18E29326E1\n\tvalidpgpkeys = 5C251B5FC54EB2F80F407AAAC54CA336CFEB557E\n\tsha512sums = 098f5800149b6d51c569ea33d126b7ce901a886b7fcebab26138089f1c33e1cf1e8489978e9ae724a568c5e198a5ebc3df3889ffaa0559a0a5332af4242d1837\n\tsha512sums = 78065bde708118b7d6e4ed492e096c763e4679a1c54bd98750d5d609d8cc2f1373023f308880f14fc923ae7f9fea34824917ef884c0f996b1f43d08ef022c0fb\n\tsha512sums = 61032d29241b74a0f28446f8cf1be0e8ec46d0847a61dadb2a4f096e8686d5f57fe5c72bcf386003f6520bc4b5856c32d63bf3efe7eb0bc0deefc9f68159e648\n\tsha512sums = c416e2121df83067376bcaacb58c05b01990f4614ad9de657d74b6da3efa441af251d13bf21e3f0f71ddcb4c9ea658b81da3d915667dc5c309c87ec32a1cb5a5\n\tsha512sums = 5a1d78b5170da5abe3d18fdf9f2c3a4d78f15ba7d1ee9ec2708c4c9c2e28973469bc19386f70b3cf32ffafbe4fcc4303e5ebbd6d5187a1df3314ae0965b25e75\n\tsha512sums = b90c99d768dc2a4f020ba854edf45ccf1b86a09d2f66e475de21fe589ff7e32c33ef4aa0876d7f1864491488fd7edb2682fc0d68e83a6d4890a0778dc2d6fe19\n\tsha512sums = 81baa1ae439b0f4d1f09371a82c02db06a97a4fc35545fc2654f7905b4422fc8cf085f70304919a4323f39e662df1e05aa8d977d1dde73507527abe3072c386b\n\tsha512sums = 299dcc7094ce53474521356647bdd2fb069731c08d14a872a425412fcd72da840727a23664b12d95465bf313e8e8297da31259508d1c62cc2dcea596160e21c5\n\tsha512sums = 0d6bc3d928cfafe4e4e0bc04dbb95c5d2b078573e4f9e0576e7f53a8fab08a7077202f575d74a3960248c4904b5f7f0661bf17dbe163c524ab51dd30e3cb80f7\n\tsha512sums = 2b50b25e8680878f7974fa9d519df7e141ca11c4bfe84a92a5d01bb193f034b1726ea05b3c0030bad1fbda8dbb78bf1dc7b73859053581b55ba813c39b27d9dc\n\tsha512sums = a436d3f5126c6c0d6b58c6865e7bd38dbfbfb7babe017eeecb5e9d162c21902cbf4e0a68cf3ac2f99815106f9fa003b075bd2b4eb5d16333fa913df6e2f3e32a\n\tsha512sums = 190112e38d5a5c0ca91b89cd58f95595262a551530a16546e1d84700fc9644aa2ca677953ffff655261e8a7bff6e6af4e431424df5f13c00bc90b77c421bc32d\n\tsha512sums = a1661ab946c6cd7d3c6251a2a9fd68afe231db58ce33c92c42594aedb5629be8f299ba08a34713327b373a3badd1554a150343d8d3e5dfb102999c281bd49154\n\tsha512sums = f6b154fdc612916d7788720cf703e34255b43ba2d19413de5f3f63f07508f4ce561ca138f987c2118c7128e1dfb01976b0ac7d5efee4d9ebaadd180e70fa013e\n\tsha512sums = 9426829605bbb9e65002437e02ed54e35c20fdf94706770a3dc1049da634147906d6b98bf7f5e7516c84068396a12c6feaf72f92b51bdf19715e0f64620319de\n\tsha512sums = da7a97d5d3701c70dd5388b0440da39006ee4991ce174777931fea2aa8c90846a622b2b911f02ae4d5fffb92680d9a7e211c308f0f99c04896278e2ee0d9a4dc\n\tsha512sums = a50d202a9c2e91a4450b45c227b295e1840cc99a5e545715d69c8af789ea3dd95a03a30f050d52855cabdc9183d4688c1b534eaa755ebe93616f9d192a855ee3\n\tsha512sums = 825b9dd0167c072ba62cabe0677e7cd20f2b4b850328022540f122689d8b25315005fa98ce867cf6e7460b2b26df16b88bb3b5c9ebf721746dce4e2271af7b97\n\npkgname = systemd\n\tpkgdesc = system and service manager\n\tinstall = systemd.install\n\tlicense = LGPL-2.1-or-later\n\tlicense = CC0-1.0\n\tlicense = GPL-2.0-or-later\n\tlicense = MIT-0\n\tdepends = systemd-libs=257.6\n\tdepends = acl\n\tdepends = libacl.so\n\tdepends = bash\n\tdepends = cryptsetup\n\tdepends = libcryptsetup.so\n\tdepends = dbus\n\tdepends = dbus-units\n\tdepends = kbd\n\tdepends = kmod\n\tdepends = hwdata\n\tdepends = libcap\n\tdepends = libcap.so\n\tdepends = libgcrypt\n\tdepends = libxcrypt\n\tdepends = libcrypt.so\n\tdepends = libidn2\n\tdepends = lz4\n\tdepends = pam\n\tdepends = libelf\n\tdepends = libseccomp\n\tdepends = libseccomp.so\n\tdepends = util-linux\n\tdepends = libblkid.so\n\tdepends = libmount.so\n\tdepends = xz\n\tdepends = pcre2\n\tdepends = audit\n\tdepends = libaudit.so\n\tdepends = openssl\n\tdepends = libcrypto.so\n\tdepends = libssl.so\n\toptdepends = libmicrohttpd: systemd-journal-gatewayd and systemd-journal-remote\n\toptdepends = quota-tools: kernel-level quota management\n\toptdepends = systemd-sysvcompat: symlink package to provide sysvinit binaries\n\toptdepends = systemd-ukify: combine kernel and initrd into a signed Unified Kernel Image\n\toptdepends = polkit: allow administration as unprivileged user\n\toptdepends = curl: systemd-journal-upload, machinectl pull-tar and pull-raw\n\toptdepends = gnutls: systemd-journal-gatewayd and systemd-journal-remote\n\toptdepends = qrencode: show QR codes\n\toptdepends = iptables: firewall features\n\toptdepends = libarchive: convert DDIs to tarballs\n\toptdepends = libbpf: support BPF programs\n\toptdepends = libpwquality: check password quality\n\toptdepends = libfido2: unlocking LUKS2 volumes with FIDO2 token\n\toptdepends = libp11-kit: support PKCS#11\n\toptdepends = tpm2-tss: unlocking LUKS2 volumes with TPM2\n\tprovides = nss-myhostname\n\tprovides = systemd-tools=257.6\n\tprovides = udev=257.6\n\tconflicts = nss-myhostname\n\tconflicts = systemd-tools\n\tconflicts = udev\n\treplaces = nss-myhostname\n\treplaces = systemd-tools\n\treplaces = udev\n\tbackup = etc/pam.d/systemd-user\n\tbackup = etc/systemd/coredump.conf\n\tbackup = etc/systemd/homed.conf\n\tbackup = etc/systemd/journald.conf\n\tbackup = etc/systemd/journal-remote.conf\n\tbackup = etc/systemd/journal-upload.conf\n\tbackup = etc/systemd/logind.conf\n\tbackup = etc/systemd/networkd.conf\n\tbackup = etc/systemd/oomd.conf\n\tbackup = etc/systemd/pstore.conf\n\tbackup = etc/systemd/resolved.conf\n\tbackup = etc/systemd/sleep.conf\n\tbackup = etc/systemd/system.conf\n\tbackup = etc/systemd/timesyncd.conf\n\tbackup = etc/systemd/user.conf\n\tbackup = etc/udev/iocost.conf\n\tbackup = etc/udev/udev.conf\n\npkgname = systemd-libs\n\tpkgdesc = systemd client libraries\n\tlicense = LGPL-2.1-or-later\n\tlicense = CC0-1.0\n\tlicense = GPL-2.0-or-later WITH Linux-syscall-note\n\tdepends = glibc\n\tdepends = gcc-libs\n\tdepends = libcap\n\tdepends = libgcrypt\n\tdepends = lz4\n\tdepends = xz\n\tdepends = zstd\n\tprovides = libsystemd\n\tprovides = libsystemd.so\n\tprovides = libudev.so\n\tconflicts = libsystemd\n\treplaces = libsystemd\n\npkgname = systemd-resolvconf\n\tpkgdesc = systemd resolvconf replacement (for use with systemd-resolved)\n\tdepends = systemd=257.6\n\tprovides = openresolv\n\tprovides = resolvconf\n\tconflicts = resolvconf\n\npkgname = systemd-sysvcompat\n\tpkgdesc = sysvinit compat for systemd\n\tdepends = systemd=257.6\n\tconflicts = sysvinit\n\npkgname = systemd-tests\n\tpkgdesc = systemd tests\n\tdepends = systemd=257.6\n\npkgname = systemd-ukify\n\tpkgdesc = Combine kernel and initrd into a signed Unified Kernel Image\n\tdepends = systemd=257.6\n\tdepends = binutils\n\tdepends = python-cryptography\n\tdepends = python-pefile\n\toptdepends = python-pillow: Show the size of splash image\n\toptdepends = sbsigntools: Sign the embedded kernel\n\tprovides = ukify\n";
        let pkgs = parse(srcinfo).unwrap();
        assert_eq!(
            pkgs,
            vec![
                Pkg {
                    pkgname: "systemd".to_string(),
                    pkgver: "257.6".to_string(),
                    pkgrel: "1".to_string(),
                    epoch: None,
                    arch: "x86_64".to_string(),
                },
                Pkg {
                    pkgname: "systemd-libs".to_string(),
                    pkgver: "257.6".to_string(),
                    pkgrel: "1".to_string(),
                    epoch: None,
                    arch: "x86_64".to_string(),
                },
                Pkg {
                    pkgname: "systemd-resolvconf".to_string(),
                    pkgver: "257.6".to_string(),
                    pkgrel: "1".to_string(),
                    epoch: None,
                    arch: "x86_64".to_string(),
                },
                Pkg {
                    pkgname: "systemd-sysvcompat".to_string(),
                    pkgver: "257.6".to_string(),
                    pkgrel: "1".to_string(),
                    epoch: None,
                    arch: "x86_64".to_string(),
                },
                Pkg {
                    pkgname: "systemd-tests".to_string(),
                    pkgver: "257.6".to_string(),
                    pkgrel: "1".to_string(),
                    epoch: None,
                    arch: "x86_64".to_string(),
                },
                Pkg {
                    pkgname: "systemd-ukify".to_string(),
                    pkgver: "257.6".to_string(),
                    pkgrel: "1".to_string(),
                    epoch: None,
                    arch: "x86_64".to_string(),
                }
            ]
        );
    }

    #[test]
    fn test_parse_epoch() {
        let srcinfo = "pkgbase = rust\n\tpkgdesc = Systems programming language focused on safety, speed and concurrency\n\tpkgver = 1.87.0\n\tpkgrel = 1\n\tepoch = 1\n\turl = https://www.rust-lang.org/\n\tarch = x86_64\n\tlicense = Apache-2.0 OR MIT\n\tcheckdepends = gdb\n\tcheckdepends = procps-ng\n\tmakedepends = clang\n\tmakedepends = cmake\n\tmakedepends = lib32-gcc-libs\n\tmakedepends = lib32-glibc\n\tmakedepends = libffi\n\tmakedepends = lld\n\tmakedepends = llvm\n\tmakedepends = musl\n\tmakedepends = ninja\n\tmakedepends = perl\n\tmakedepends = python\n\tmakedepends = rust\n\tdepends = bash\n\tdepends = curl\n\tdepends = gcc\n\tdepends = gcc-libs\n\tdepends = glibc\n\tdepends = libssh2\n\tdepends = llvm-libs\n\tdepends = openssl\n\tdepends = zlib\n\toptions = !emptydirs\n\toptions = !lto\n\tsource = https://static.rust-lang.org/dist/rustc-1.87.0-src.tar.gz\n\tsource = https://static.rust-lang.org/dist/rustc-1.87.0-src.tar.gz.asc\n\tsource = 0001-bootstrap-Change-libexec-dir.patch\n\tsource = 0002-bootstrap-Change-bash-completion-dir.patch\n\tsource = 0003-bootstrap-Use-lld-mode-only-for-host-linked-targets.patch\n\tsource = 0004-compiler-Change-LLVM-targets.patch\n\tsource = 0005-compiler-Use-wasm-ld-for-wasm-targets.patch\n\tsource = 0006-compiler-Use-aarch64-linux-gnu-gcc-to-link-aarch64-t.patch\n\tvalidpgpkeys = 108F66205EAEB0AAA8DD5E1C85AB96E6FA1BE5FE\n\tb2sums = cb104202691697ae263dc688067420dbe50023aef56dd7d4de2e662c95f17ee888e3ff1a3513b268e05243b90e51e73ba8fe772a97dba0c6119910e99e805bbe\n\tb2sums = SKIP\n\tb2sums = 743bf583cf40ec856b7d89953af53db9ef9e7047b936a60e88e626bb7b389446bc5ed7b8b1d02001131ec728455a77ed78d6eeb64e61c370de81c2fe488836cc\n\tb2sums = f39eef3c8dce60f2ecdac5ac169cbe749c8c353241a1c48cf4815f1da37be174007d5efc98574b4865dcaee886b618139d54fc274be92cc8063a9c48f48b985b\n\tb2sums = 188c7065d8a623396d7ed93c1f2a4188fd46ede9f93e9bd8e933648fc3db0924b68ec78d2f4b29529bbdcc5d809f6711438535e70d153e0131f10273109e162b\n\tb2sums = 7c919047f50325be6ad5686094c97affe2148f99c5b41748e8e0187674e3b2c0eed10cd3798c866fbd2f65889636f88a439bad0892c093f6bece3f1263c89c30\n\tb2sums = 2317343e6b986d1ec1fb6d035fb6d8933245704b5be1b3e4a032ad14300d8a338087c52e53a6dff4ceda52232ce7f21dd8ad536c9d4da04faee6a9b79a9670b6\n\tb2sums = e9f6b2d58e2a845d8841c0eb2dbde1d903bb6bed1871d090cf8928fe4a2dd5ece0cf157f3f263cf980d1dba7fd9c47565340bc1e19ecd2f28ffb297fe70da30d\n\npkgname = rust\n\toptdepends = gdb: rust-gdb script\n\toptdepends = lldb: rust-lldb script\n\tprovides = cargo\n\tprovides = rustfmt\n\tconflicts = cargo\n\tconflicts = rust-docs<1:1.56.1-3\n\tconflicts = rustfmt\n\treplaces = cargo\n\treplaces = cargo-tree\n\treplaces = rust-docs<1:1.56.1-3\n\treplaces = rustfmt\n\npkgname = lib32-rust-libs\n\tpkgdesc = 32-bit target and libraries for Rust\n\tdepends = lib32-gcc-libs\n\tdepends = lib32-glibc\n\tdepends = rust\n\tprovides = lib32-rust\n\tconflicts = lib32-rust\n\treplaces = lib32-rust\n\npkgname = rust-musl\n\tpkgdesc = Musl target for Rust\n\tdepends = rust\n\npkgname = rust-src\n\tpkgdesc = Source code for the Rust standard library\n\tdepends = rust\n";
        let pkgs = parse(srcinfo).unwrap();
        assert_eq!(
            pkgs,
            vec![
                Pkg {
                    pkgname: "rust".to_string(),
                    pkgver: "1.87.0".to_string(),
                    pkgrel: "1".to_string(),
                    epoch: Some("1".to_string()),
                    arch: "x86_64".to_string()
                },
                Pkg {
                    pkgname: "lib32-rust-libs".to_string(),
                    pkgver: "1.87.0".to_string(),
                    pkgrel: "1".to_string(),
                    epoch: Some("1".to_string()),
                    arch: "x86_64".to_string()
                },
                Pkg {
                    pkgname: "rust-musl".to_string(),
                    pkgver: "1.87.0".to_string(),
                    pkgrel: "1".to_string(),
                    epoch: Some("1".to_string()),
                    arch: "x86_64".to_string()
                },
                Pkg {
                    pkgname: "rust-src".to_string(),
                    pkgver: "1.87.0".to_string(),
                    pkgrel: "1".to_string(),
                    epoch: Some("1".to_string()),
                    arch: "x86_64".to_string()
                }
            ]
        );
    }
}
