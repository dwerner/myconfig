use directories::UserDirs;

use myconfig::common::AppError;
use myconfig::packages::PackagesConfig;

fn init_logger() {
    use simplelog::*;
    use std::fs::File;
    let _ = CombinedLogger::init(vec![
        TermLogger::new(
           LevelFilter::Info,
           Config::default(),
           TerminalMode::Mixed
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("myconfig.log").unwrap()
        ),
    ]);
}

// TODO
// - gather configs, save and commit to repo
// - watch configs for changes

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    init_logger();
    log::info!("my config starting up");

    // dotfiles.yml
    // load and symlink

    let force_update = false;
    let user_dirs = UserDirs::new().ok_or_else(|| AppError::NoUserDirs)?;
    let home_dir = user_dirs.home_dir().to_str().map(str::to_string).ok_or_else(|| AppError::NoHomeDir)?;
    log::info!("TODO: get dotfiles from/ install dotfiles to home dir {:?}", home_dir);

    let packages = async_std::fs::read_to_string("packages.yaml").await?;
    let p = serde_yaml::from_str::<PackagesConfig>(&packages)?;

    p.install_ppas()?;
    p.install_apt_packages()?;
    p.install_downloaded_debs(force_update)?;
    p.install_cargo_packages()?;
    p.install_direct_curls(&home_dir)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_ls() {
        let ls = which("ls");
        assert_eq!(
            Some(PathBuf::from("/usr/bin/ls\n")), ls);
    }
}
