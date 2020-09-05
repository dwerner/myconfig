use directories::UserDirs;

///
/// myconfigs is intended to act as a more robust dotfiles - it gathers configs, watches for
/// changes, commits to it's own repo, and provides an automated setup tool. (TODO)

macro_rules! sudo {
    ($e:expr) => {{ // accept an expression which derefs to the input to Command::args
        std::process::Command::new("sudo")
            .args(&$e)
            .status()
    }};
    ($($e:ident)+) => {{ // accept bare idents (only works for legal idents) and stringify as command
        std::process::Command::new("sudo")
            .args(&[$( stringify!( $e ) ),+])
            .status()
    }};
    ($($e:ident)+, $l:expr) => {{ // accept bare idents, stringify each as args list and append with expr list
        std::process::Command::new("sudo")
            .args(&[$( stringify!( $e ) ),+])
            .args($l)
            .status()
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
    apt: Vec<String>,
    ppas: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("Unable to get user's dirs")]
    NoUserDirs,
}

// TODO
// - gather configs, save and commit to repo
// - watch configs for changes

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    init_logger();
    log::info!("Hello, world!");
    let user_dirs = UserDirs::new().ok_or_else(|| AppError::NoUserDirs)?;
    log::info!("{:?}", user_dirs.home_dir());

    let packages = async_std::fs::read_to_string("packages.yaml").await?;
    let cfg = serde_yaml::from_str::<Config>(&packages)?;

    for ppa in cfg.ppas.iter() {
        let ppa_str = if ppa.starts_with("ppa:") {
            ppa.clone()
        } else {
            format!("ppa:{}", ppa)
        };
        let status = sudo!(["add-apt-repository", "-y", &ppa_str])?;
        log::debug!("added ppa {} - status {}", ppa, status);
    }
    log::debug!("after adding ppas, we run apt update");
    sudo!(apt update)?;

    log::info!("Installing packages {:?}", cfg.apt);
    sudo!(apt install sudo, cfg.apt)?;

    Ok(())
}
