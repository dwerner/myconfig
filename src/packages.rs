use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::{cmd, common, sudo};

#[derive(serde_derive::Deserialize, Debug)]
pub struct PackagesConfig {
    #[serde(default)]
    pub dry_run: bool,

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

    #[serde(alias = "dest", default = "Download::default_destination")]
    pub dst: String,
}

impl Download {
    fn default_destination() -> String {
        "temp_install".to_string()
    }
}

impl PackagesConfig {
    pub fn install_ppas(&self) -> anyhow::Result<()> {
        log::info!("Installing ppas...");
        for ppa in self.ppas.iter() {
            let ppa_str = normalize_ppa_str(ppa);
            let cmd = sudo!(&["add-apt-repository", "-y", &ppa_str]);
            self.exec_or_log_command(cmd)?;
        }
        log::debug!("after adding ppas, we run apt update");
        self.exec_or_log_command(sudo!(apt update, ["-qq"]))?;
        Ok(())
    }

    pub fn install_apt_packages(&self) -> anyhow::Result<()> {
        log::info!("Installing apt packages...");
        let installed = common::find_installed(&self.apt);
        if !installed.is_empty() {
            log::info!("already installed (will skip): {:?}", installed);
        }
        let not_installed = common::find_not_installed(&self.apt);
        if not_installed.is_empty() {
            log::info!("all apt pkgs already installed, skipping...");
            return Ok(());
        }

        let mut apt = vec!["apt", "-qqy", "install"];
        log::info!("Installing apt packages -> {:?}", not_installed);
        for pkg in not_installed.iter() {
            apt.push(pkg);
        }
        self.exec_or_log_command(sudo!(apt))?;
        Ok(())
    }

    pub fn install_direct_curls(&self, user_home_dir: impl AsRef<Path>) -> anyhow::Result<()> {
        for Download { name, url, dst } in self.curl.iter() {
            let mut user_home_dest = PathBuf::from(user_home_dir.as_ref());
            user_home_dest.push(if dst.is_empty() {
                Download::default_destination()
            } else {
                dst.to_owned()
            });

            let filename = user_home_dest
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("unable to get path to destination"))?;

            log::info!(
                "direct curl {} : {} -downloaded to-> {}",
                name,
                url,
                filename
            );
            self.exec_or_log_command(cmd!(["curl", "-L", "-o", &filename, &url]))?;
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
                    let curl = cmd!(["curl", "-L", "-o", &filename, &url]);
                    self.exec_or_log_command(curl)?;

                    log::info!("installing {}", filename);
                    let apt_install = sudo!(apt install, ["-y", &filename]);
                    self.exec_or_log_command(apt_install)?;
                }
            }
        }
        Ok(())
    }

    fn exec_or_log_command(
        &self,
        mut apt_install: std::process::Command,
    ) -> Result<(), anyhow::Error> {
        if self.dry_run {
            log::info!("dry_run: {:?}", apt_install);
        } else {
            let status = apt_install.status()?;
            log::info!(
                "ran command {:?} exited with status {}",
                apt_install,
                status
            );
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
        self.exec_or_log_command(cmd!(cargo))?;
        Ok(())
    }
}

fn normalize_ppa_str(ppa: &String) -> String {
    let ppa_str = if ppa.starts_with("ppa:") {
        ppa.clone()
    } else {
        format!("ppa:{}", ppa)
    };
    ppa_str
}
