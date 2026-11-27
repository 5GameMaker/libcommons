# libcommons

Rust utils I don't want to be bothered rewriting.

## Disclaimer

`libcommons` has no API stability and it can randomly change whenever. No bugs
will be preserved unless it's too annoying to fix.

## MSRV

Latest Rust I have installed on my devices. Some features (i.e. matrix) require
Nightly.

## Dependency count

For this crate we'll only be adding 2 types of features:

- Crates like `libc` or `windows`, required (unless we want to manually `extern "C"` every
  struct and function from them) to interact with the underlying operating system.

  Those crates will always be included with `version = "*"` as they're unlikely to ever change and
  that prevents libcommons from adding a yet another version of those crates, when you
  already have like 5 of them cuz other crates always pin their versions.
- Crates like `serde` or `bytemuck` in order to add compatibility with those crates

  Those crates will always be included with `optional = true` as if you don't care about them,
  there's no need to have to download them. Ideally for crates like `bytemuck`, libcommons
  must already implement that functionality on its own, and if ever needed, is likely to
  be implemented in the future.

## Installation

Add to `Cargo.toml`:

- `stable`
`libcommons = "0.8.0"`

- `master`
> `master` may sometimes not compile and contain WIP features that don't work.
> Whenever a feature is considered "good enough", it's pushed to `stable`.
> 
> There is no reason to ever depend on `master` branch, but if you want to suffer:

`libcommons = { git = "https://github.com/5GameMaker/libcommons" }`

Specify features you want. This is important because some of them require `nightly`.

Also never specify `all-features=true`. Some of the features in libcommons depend on those of Rust
that either don't work anymore or no longer exist, thus libcommons will not compile.

Use `--features=full` or `--features=full-nightly` instead.
