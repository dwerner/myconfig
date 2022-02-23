use std::path::Path;

use async_std::fs;
use directories::UserDirs;

use myconfig::common::{version, AppError};
use myconfig::dotfiles::DotFilesConfig;
use myconfig::packages::PackagesConfig;
use structopt::StructOpt;

fn init_logger() {
    use simplelog::*;
    use std::fs::File;
    let _ = CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("myconfig.log").unwrap(),
        ),
    ]);
}

#[derive(Debug, StructOpt)]
enum Commands {
    Install {
        #[structopt(long)]
        dry_run: bool,

        #[structopt(long)]
        force: bool,
    },
    Check {},
}

// TODO
// - gather configs, save and commit to repo
// - watch configs for changes

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    init_logger();
    log::info!("my config starting up");

    let user_dirs = UserDirs::new().ok_or_else(|| AppError::NoUserDirs)?;
    let home_dir = user_dirs
        .home_dir()
        .to_str()
        .map(str::to_string)
        .ok_or_else(|| AppError::NoHomeDir)?;

    let command: Commands = StructOpt::from_args();

    let dotfiles = parse_dotfiles().await?;
    let mut packages = parse_packages().await?;

    match command {
        Commands::Install { force, dry_run } => {
            packages.dry_run = dry_run;
            install_all(home_dir, force, packages)?
        }
        Commands::Check {} => check_all(home_dir, dotfiles, &packages)?,
    }
    Ok(())
}

async fn parse_packages() -> Result<PackagesConfig, anyhow::Error> {
    let packages = fs::read_to_string("packages.yaml").await?;
    Ok(serde_yaml::from_str::<PackagesConfig>(&packages)?)
}

async fn parse_dotfiles() -> Result<DotFilesConfig, anyhow::Error> {
    let dotfiles = fs::read_to_string("dotfiles.yaml").await?;
    Ok(serde_yaml::from_str::<DotFilesConfig>(&dotfiles)?)
}

fn install_all(
    home_dir: impl AsRef<Path>,
    force_update: bool,
    packages: PackagesConfig,
) -> Result<(), anyhow::Error> {
    log::info!(
        "TODO: get dotfiles from/ install dotfiles to home dir {:?}",
        home_dir.as_ref()
    );
    packages.install_ppas()?;
    packages.install_apt_packages()?;
    packages.install_downloaded_debs(force_update)?;
    packages.install_cargo_packages()?;
    packages.install_direct_curls(&home_dir)?;
    Ok(())
}

fn check_all(
    home_dir: impl AsRef<Path>,
    dotfiles: DotFilesConfig,
    packages: &PackagesConfig,
) -> Result<(), anyhow::Error> {
    for df in dotfiles.symlinks {
        if let Err(err) = df.check(&home_dir) {
            println!("error getting symlink file metadata {} {:?}", df.src, err);
        }
    }
    log::info!("Checking packages...");
    for download in packages.ppas.iter() {
        log::info!("ppa: {}", download);
    }
    for download in packages.apt.iter() {
        log::info!("apt: {}", download);
    }
    for download in packages.cargo.iter() {
        log::info!("cargo: {}", download);
    }
    for download in packages.debs.iter() {
        log::info!("manual deb: {:?}", download);
    }
    for download in packages.curl.iter() {
        log::info!("manual curl: {:?}", download);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use myconfig::common::which;

    #[test]
    fn should_find_ls() {
        let ls = which("ls");
        assert_eq!(Some(PathBuf::from("/usr/bin/ls\n")), ls);
    }
}
