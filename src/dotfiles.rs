use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::cmd;
use crate::common::symlink;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DotFilesConfig {
    pub symlinks: Vec<DotFile>,
}

#[derive(Serialize, Deserialize)]
pub struct DotFile {
    pub name: String,
    pub src: String,
    pub dst: String,
}

impl DotFile {
    pub fn check(&self, user_home: impl AsRef<Path>) -> anyhow::Result<()> {
        let mut home_dest = PathBuf::from(user_home.as_ref());
        home_dest.push(self.dst.clone());
        let src_meta = fs::metadata(&self.src)?;
        let dst_meta = fs::metadata(&home_dest)?;
        log::info!("symlink present: {:?} -> {}", home_dest.to_str(), self.src);
        log::info!("symlink metadata: {:?} -> {:?}", dst_meta, src_meta);
        Ok(())
    }
}
