# PlatypOS

This is a toy unix-like project exploring the following:

- Prefer Rust over C when possible
    - Gnu/coreutils replaced with [uutils/coreutils](https://github.com/uutils/coreutils)
    - OpenSSL partially replaced with [rustls-ffi](https://github.com/rustls/rustls-ffi) (most prominently in libcurl)
    - [Rust](https://github.com/rust-lang/rust) and [cargo-c](https://github.com/lu-zero/cargo-c) as first-class citizens
    - Tor replaced with [Arti](https://gitlab.torproject.org/tpo/core/arti)
    - Nginx replaced with [miniserve](https://github.com/svenstaro/miniserve)
    - Gnupg partially replaced with [sequoia-sq](https://gitlab.com/sequoia-pgp/sequoia-sq)
- Reproducible Builds
- Contribution through pull requests

It heavily relies on prior work by [Arch Linux](https://archlinux.org/) and
their packagers/developers, many `PKGBUILD`s are yoinked without modification. 🖤

## Building packages

There's no standard tooling yet.

```sh
cd os/librustls
makepkg -rsfC
```

## Generating the database

Once all packages have been built, you can generate a database file. This
process utilizes the `.SRCINFO` files from your git repository and gathers
additional details from the compiled `.pkg.tar.zst` packages.

```sh
cd db/
cargo run --release -- make ../os -o /var/www/repo/main/os/x86_64/ -A x86_64 -n main -v
```

## Trivia

The following issues have been identified as part of this project:

- [`uutils/coreutils#8033`](https://github.com/uutils/coreutils/issues/8033) - autotools+audit-userspace fails to `make install` due to multiple `-m` arguments
- [`uutils/coreutils#8044`](https://github.com/uutils/coreutils/issues/8044) - `mv` tries to follow dangling symlink when destination is a directory
- [`archlinux/packages/musl#1`](https://gitlab.archlinux.org/archlinux/packaging/packages/musl/-/issues/1) - security patches not correctly applied for x86_64

## License

`0BSD`
