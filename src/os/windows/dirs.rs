use std::env::home_dir;

use crate::fs::{DirRel, DirType};

#[allow(deprecated)]
pub fn dir(rel: DirRel, ty: DirType) -> Option<PathBuf> {
    match (ty, rel) {
        (DirType::Home, DirRel::User) => home_dir(),
        _ => todo!(),
    }
}
