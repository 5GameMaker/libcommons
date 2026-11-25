use std::{fs::File, io::{BufWriter, Write}, path::Path, process::Command};

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let mut out_file = BufWriter::new(
        File::create(Path::new(&out_dir).join("gen.rs")).unwrap());

    let rustc = String::from_utf8(Command::new(
            std::env::var_os("RUSTC").unwrap())
        .arg("--version").output().unwrap().stdout).unwrap();

    let version = rustc.strip_prefix("rustc ").unwrap().split_once(' ').unwrap().0;
    let (version_s, version_tag) = version.split_once('-').unwrap_or((version, ""));
    let mut version_it = version_s.split('.');

    let version_major = version_it.next().and_then(|x| x.parse::<i32>().ok()).unwrap();
    let version_minor = version_it.next().and_then(|x| x.parse::<i32>().ok()).unwrap();
    let version_patch = version_it.next().and_then(|x| x.parse::<i32>().ok()).unwrap();
    assert!(version_it.next().is_none());

    if std::env::var_os("CARGO_FEATURE_META").is_some() {
        writeln!(out_file, "/// The entire string returned by `$ rustc --version`").unwrap();
        writeln!(out_file, "pub static RUSTC_VERSION_STRING: &str = {:?};", rustc.trim()).unwrap();
        writeln!(out_file, "/// Just the version part of `$ rustc --version`").unwrap();
        writeln!(out_file, "pub static RUSTC_VERSION_WITH_TAG: &str = {version:?};").unwrap();
        writeln!(out_file, "/// The version of rustc libcommons was compiled against").unwrap();
        writeln!(out_file, "pub static RUSTC_VERSION_ONLY: &str = {version_s:?};").unwrap();
        writeln!(out_file, "/// Rustc version tag").unwrap();
        writeln!(out_file, "///").unwrap();
        writeln!(out_file, "/// May be empty.").unwrap();
        writeln!(out_file, "pub static RUSTC_VERSION_TAG: &str = {version_tag:?};").unwrap();
        writeln!(out_file, "pub static RUSTC_VERSION_MAJOR: i32 = {version_major};").unwrap();
        writeln!(out_file, "pub static RUSTC_VERSION_MINOR: i32 = {version_minor};").unwrap();
        writeln!(out_file, "pub static RUSTC_VERSION_PATCH: i32 = {version_patch};").unwrap();
    }

    println!("cargo::rustc-check-cfg=cfg(libcommons_rustc_nightly, values(\"y\"))");
    if version_tag == "nightly" {
        println!("cargo::rustc-cfg=libcommons_rustc_nightly=\"y\"");
    }

    // Just in case!
    for i in 1..=100 {
        println!("cargo::rustc-check-cfg=cfg(libcommons_rustc_min_{i}, values(any()))");
    }

    for i in 1..version_major {
        for o in 1..=1000 {
            println!("cargo::rustc-cfg=libcommons_rustc_min_{i}=\"{o}\"");
        }
    }

    for i in 1..=version_minor {
        println!("cargo::rustc-cfg=libcommons_rustc_min_{version_major}=\"{i}\"");
    }

    out_file.flush().unwrap();
}
