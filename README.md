# PlatypOS

This is a toy unix-like project exploring the following:

- Prefer Rust over C when possible
- Reproducible Builds
- Contribution through pull requests

It heavily relies on prior work by [Arch Linux](https://archlinux.org/) and
their packagers/developers, many `PKGBUILD`s are yoinked without modification.

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
cargo run --release -- ../os -v -o ../main.db
```

## Trivia

The following issues have been identified as part of this project:

- [`uutils/coreutils#8033`](https://github.com/uutils/coreutils/issues/8033) - autotools+audit-userspace fails to `make install` due to multiple `-m` arguments

## License

`0BSD`
