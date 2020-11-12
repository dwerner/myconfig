# myconfig

Just a silly reimplementation of my dotfiles configuration repository as a rust project.

`sh bootstrap.sh` 

`myconfig` is intended to act as a more robust dotfiles - it gathers configs, watches for
changes, commits to it's own repo, and provides an automated setup tool. (TODO)

`packages.yaml` defines packages to be installed.

`dotfiles.yaml` defines various dotfiles to be symlinked to their respective locations.
