# UniPager v1

Universal POCSAG transmitter controller written in Rust.

This fork offers continued support for from source builds of UniPager v1.

See documentation on the [original repo](https://github.com/rwth-afu/UniPager) and [wiki](https://hampager.de/dokuwiki/doku.php?id=unipagerenglish).

## Compiling for ARM

The following will compile UniPager for ARM 64 using MUSL libc.
This is suitable for deployment on [Alpine Linux](https://alpinelinux.org/).

```sh
cargo install cross
cross build --target aarch64-unknown-linux-musl
```
