use std::{
    env::{home_dir, var},
    ffi::c_int,
    path::PathBuf,
    str::FromStr,
};

use crate::fs::{DirRel, DirType};

unsafe extern "C" {
    safe fn getuid() -> c_int;
}

#[allow(deprecated)]
pub fn dir(rel: DirRel, ty: DirType) -> Option<PathBuf> {
    match (ty, rel) {
        (DirType::Home, DirRel::User) => home_dir(),
        (DirType::Home, DirRel::System) => Some("/var/lib".into()),
        (DirType::Runtime, DirRel::User) => {
            if let Some(x) = var("XDG_RUNTIME_DIR")
                .ok()
                .and_then(|x| PathBuf::from_str(&x).ok())
            {
                return Some(x);
            }
            Some(format!("/run/user/{}", getuid()).into())
        }
        (DirType::Runtime, DirRel::System) => Some("/run".into()),
        (DirType::Share, DirRel::User) => {
            if let Some(x) = var("XDG_DATA_HOME")
                .ok()
                .and_then(|x| PathBuf::from_str(&x).ok())
            {
                return Some(x);
            }
            home_dir().map(|x| x.join(".local/share"))
        }
        (DirType::Share, DirRel::System) => Some("/usr/share".into()),
        (DirType::Cache, DirRel::User) => {
            if let Some(x) = var("XDG_CACHE_HOME")
                .ok()
                .and_then(|x| PathBuf::from_str(&x).ok())
            {
                return Some(x);
            }
            home_dir().map(|x| x.join(".cache"))
        }
        (DirType::Cache, DirRel::System) => Some("/tmp".into()),
        (DirType::State, DirRel::User) => {
            if let Some(x) = var("XDG_STATE_HOME")
                .ok()
                .and_then(|x| PathBuf::from_str(&x).ok())
            {
                return Some(x);
            }
            home_dir().map(|x| x.join(".local/state"))
        }
        (DirType::State, DirRel::System) => None,
        (DirType::Bin, DirRel::User) => home_dir().map(|x| x.join(".local/bin")),
        (DirType::Bin, DirRel::System) => Some("/usr/bin".into()),
        (DirType::Lib, DirRel::User) => None,
        (DirType::Lib, DirRel::System) => Some("/usr/bin".into()),
        (DirType::Config, DirRel::User) => {
            if let Some(x) = var("XDG_CONFIG_HOME")
                .ok()
                .and_then(|x| PathBuf::from_str(&x).ok())
            {
                return Some(x);
            }
            home_dir().map(|x| x.join(".config"))
        }
        (DirType::Config, DirRel::System) => Some("/etc".into()),
    }
}
