use std::path::PathBuf;
use std::collections::HashSet;
use directories::UserDirs;

///
/// myconfigs is intended to act as a more robust dotfiles - it gathers configs, watches for
/// changes, commits to it's own repo, and provides an automated setup tool. (TODO)
///
///

fn which(progname: &str) -> Option<PathBuf> {
    let out: std::process::Output = std::process::Command::new("which")
        .arg(progname)
        .output().ok()?;
    let output:String = String::from_utf8_lossy(&out.stdout).into();
    let path = PathBuf::from(output);
    Some(path)
}

macro_rules! cmd {
    ($i:ident, $e:expr) => {{
        let mut cmd = std::process::Command::new(stringify!($i));
        cmd.args($e);
        cmd.status()
    }};
    ($e:expr) => {{
        let mut cmd = std::process::Command::new($e[0]);
        if $e.len() > 1 {
            cmd.args($e[1..].to_vec());
        }
        cmd.status()
    }};
}

macro_rules! sudo {
    ($e:expr) => {{ // accept an expression which derefs to the input to Command::args
        cmd!(sudo, $e)
    }};
    ($($e:ident)+) => {{ // accept bare idents (only works for legal idents) and stringify as command
        cmd!(sudo, vec![$( stringify!( $e ) ),+])
    }};
    ($($e:ident)+, $l:expr) => {{ // accept bare idents, stringify each as args list and append with expr list
        let mut args = vec![$( stringify!( $e ).to_string() ),+];
        let owned: HashSet<String> = $l.iter().map(|i| i.to_string()).collect();
        args.extend(owned);
        cmd!(sudo, args)
    }};
}

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

#[derive(serde_derive::Deserialize, Debug)]
struct Config {
    temp_dir: String,

    #[serde(default)]
    apt: HashSet<String>,

    #[serde(default)]
    ppas: HashSet<String>,

    #[serde(default)]
    debs: HashSet<NameUrl>,

    #[serde(default)]
    cargo: HashSet<String>,
}

#[derive(serde_derive::Deserialize, Debug, PartialEq, Eq, Hash)]
struct NameUrl {
    name: String,
    url: String,
}

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("Unable to get user's dirs")]
    NoUserDirs,

    #[error("Unable to find program {0}")]
    NoInstalledProg(String)
}


fn install_ppas(cfg: &Config) -> anyhow::Result<()> {
    for ppa in cfg.ppas.iter() {
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

fn install_apt_packages(cfg: &Config) -> anyhow::Result<()> {
    let not_installed = find_not_installed(&cfg.apt);
    if not_installed.is_empty() {
        log::info!("apt pkgs already installed: {:?}", cfg.cargo);
        return Ok(())
    }

    let mut apt = vec!["apt", "-qqy", "install"];
    log::info!("Installing apt packages -> {:?}", not_installed);
    for pkg in not_installed.iter() {
        apt.push(pkg);
    }
    sudo!(apt)?;
    Ok(())
}

fn install_downloaded_debs(cfg: &Config, force_update: bool) -> anyhow::Result<()> {
    log::info!("debs");
    for NameUrl{ name, url } in cfg.debs.iter() {
        match which(name) {
            Some(path) if !force_update => {
                log::info!("found {} at {:?}, no need to install", name, path);
            }
            _ => {
                log::info!("grabbing custom deb {} from url {}", name, url);
                let filename = format!("./{}/{}.deb", cfg.temp_dir, name);
                cmd!(["curl", "-L", "-o", &filename, &url])?;
                log::info!("installing {}", filename);
                sudo!(apt install, ["-y", &filename])?;
            }
        }
    }
    Ok(())
}

fn find_not_installed(exes: &HashSet<String>) -> HashSet<String> {
    exes.iter().filter(|i| which(i).is_none()).cloned().collect::<HashSet<_>>()
}

fn install_cargo_packages(cfg: &Config) -> anyhow::Result<()> {
    let not_installed = find_not_installed(&cfg.cargo);
    if not_installed.is_empty() {
        log::info!("cargo pkgs already installed: {:?}", cfg.cargo);
        return Ok(())
    }

    log::info!("Installing cargo packages {:?}", not_installed);
    let mut cargo = vec!["cargo", "-q", "install"];
    for pkg in not_installed.iter() {
        cargo.push(pkg);
    }
    cmd!(cargo)?;
    Ok(())
}

// TODO
// - gather configs, save and commit to repo
// - watch configs for changes

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    init_logger();
    log::info!("my config starting up");

    let force_update = false;
    which("defnotfound").ok_or_else(|| AppError::NoInstalledProg("defnotfound".to_string()))?;

    let user_dirs = UserDirs::new().ok_or_else(|| AppError::NoUserDirs)?;
    log::info!("TODO: get dotfiles from/ install dotfiles to home dir {:?}", user_dirs.home_dir());

    let packages = async_std::fs::read_to_string("packages.yaml").await?;
    let cfg = serde_yaml::from_str::<Config>(&packages)?;

    install_ppas(&cfg)?;
    install_apt_packages(&cfg)?;
    install_downloaded_debs(&cfg, force_update)?;
    install_cargo_packages(&cfg)?;

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
