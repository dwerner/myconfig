use std::collections::HashSet;
use std::io::Error as IoError;
use std::path::PathBuf;

#[macro_export]
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

#[macro_export]
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

pub fn which(progname: &str) -> Option<PathBuf> {
    let out: std::process::Output = std::process::Command::new("which")
        .arg(progname)
        .output()
        .ok()?;
    let output: String = String::from_utf8_lossy(&out.stdout).into();
    if output.is_empty() {
        return None;
    }
    let path = PathBuf::from(output);
    Some(path)
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Unable to get user's dirs")]
    NoUserDirs,

    #[error("Unable to get user's home dir")]
    NoHomeDir,

    #[error("Unable to find program {0}")]
    NoInstalledProg(String),

    #[error("IoError with file {0} {1:?}")]
    IoError(String, IoError),
}

pub fn symlink(src: &str, dest: &str) -> anyhow::Result<()> {
    cmd!(ln, &["-s", src, dest])?;
    Ok(())
}

pub fn find_not_installed(exes: &HashSet<String>) -> HashSet<String> {
    exes.iter()
        .filter(|i| which(i).is_none())
        .cloned()
        .collect::<HashSet<_>>()
}
