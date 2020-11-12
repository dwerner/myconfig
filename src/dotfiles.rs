use std::fs;

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
    pub fn check(&self) -> anyhow::Result<()> {
        let dst_meta = fs::metadata(&self.dst)?;
        log::info!("metadata: {:?}", dst_meta);
        Ok(())
    }
}
