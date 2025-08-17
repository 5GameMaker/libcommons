# libcommons

Rust utils I don't want to be bothered rewriting.

## Disclaimer

`libcommons` has no API stability and it can randomly change whenever. No bugs
will be preserved unless it's too annoying to fix.

## MSRV

Latest Rust I have installed on my devices. Some features (i.e. matrix) require
Nightly.

## Installation

Add to `Cargo.toml`:

- `stable`
`commons = "0.6.0"`

- `master`
> `master` may sometimes not compile and contain WIP features that don't work.
> Whenever a feature is considered "good enough", it's pushed to `stable`.
> 
> There is no reason to ever depend on `master` branch, but if you want to suffer:

`commons = { git = "https://github.com/5GameMaker/libcommons" }`

Specify features you want. This is important because some of them require `nightly`.
