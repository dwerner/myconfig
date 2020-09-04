use async_std::fs;
use directories::UserDirs;

///
/// myconfigs is intended to act as a more robust dotfiles - it gathers configs, watches for
/// changes, commits to it's own repo, and provides an automated setup tool. (TODO)

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("Unable to get user's dirs")]
    NoUserDirs,
    #[error("unknown, but intentional error")]
    Unknown,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let user_dirs = UserDirs::new().ok_or_else(|| AppError::NoUserDirs)?;
    println!("{:?}", user_dirs.home_dir());
    Err(AppError::Unknown.into())


// TODO
// - gather configs, save and commit to repo
// - watch configs for changes

}
