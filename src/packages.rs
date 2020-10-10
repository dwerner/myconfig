use std::collections::HashSet;

use crate::{cmd, common, sudo};

#[derive(serde_derive::Deserialize, Debug)]
pub struct PackagesConfig {
    pub temp_dir: String,

    #[serde(default)]
    pub apt: HashSet<String>,

    #[serde(default)]
    pub ppas: HashSet<String>,

    #[serde(default)]
    pub debs: HashSet<Download>,

    #[serde(default)]
    pub cargo: HashSet<String>,

    #[serde(default)]
    pub curl: HashSet<Download>,
}

#[derive(serde_derive::Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Download {
    pub name: String,
    pub url: String,

    #[serde(default)]
    pub dst: String,
}

impl PackagesConfig {
    pub fn install_ppas(&self) -> anyhow::Result<()> {
        for ppa in self.ppas.iter() {
            let ppa_str = if ppa.starts_with("ppa:") {
                ppa.clone()
            } else {
                format!("ppa:{}", ppa)
            };
            let status = sudo!(&["add-apt-repository", "-y", &ppa_str])?;
            log::debug!("added ppa {} - status {}", ppa, status);
        }
        log::debug!("after adding ppas, we run apt update");
        sudo!(apt update, ["-qq"])?;
        Ok(())
    }

    pub fn install_apt_packages(&self) -> anyhow::Result<()> {
        let not_installed = common::find_not_installed(&self.apt);
        if not_installed.is_empty() {
            log::info!("apt pkgs already installed: {:?}", self.cargo);
            return Ok(());
        }

        let mut apt = vec!["apt", "-qqy", "install"];
        log::info!("Installing apt packages -> {:?}", not_installed);
        for pkg in not_installed.iter() {
            apt.push(pkg);
        }
        sudo!(apt)?;
        Ok(())
    }

    pub fn install_direct_curls(&self, user_home_dir: &str) -> anyhow::Result<()> {
        for Download { name, url, dst } in self.curl.iter() {
            let filename = format!("{}/{}", user_home_dir, dst);
            log::info!(
                "direct curl {} : {} -downloaded to-> {}",
                name,
                url,
                filename
            );
            cmd!(["curl", "-L", "-o", &filename, &url])?;
        }
        Ok(())
    }

    pub fn install_downloaded_debs(&self, force_update: bool) -> anyhow::Result<()> {
        for Download { name, url, .. } in self.debs.iter() {
            match common::which(name) {
                Some(path) if !force_update => {
                    log::info!("found {} at {:?}, no need to install", name, path);
                }
                _ => {
                    log::info!("grabbing custom deb {} from url {}", name, url);
                    let filename = format!("./{}/{}.deb", self.temp_dir, name);
                    cmd!(["curl", "-L", "-o", &filename, &url])?;
                    log::info!("installing {}", filename);
                    sudo!(apt install, ["-y", &filename])?;
                }
            }
        }
        Ok(())
    }

    pub fn install_cargo_packages(&self) -> anyhow::Result<()> {
        let not_installed = common::find_not_installed(&self.cargo);
        if not_installed.is_empty() {
            log::info!("cargo pkgs already installed: {:?}", self.cargo);
            return Ok(());
        }

        log::info!("Installing cargo packages {:?}", not_installed);
        let mut cargo = vec!["cargo", "-q", "install"];
        for pkg in not_installed.iter() {
            cargo.push(pkg);
        }
        cmd!(cargo)?;
        Ok(())
    }
}
