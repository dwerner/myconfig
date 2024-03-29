#!/usr/bin/env fish
source $HOME/.config/fish/rustenv.fish

set GPG_TTY (tty)

function parse_git_branch
  set -l branch (git branch 2> /dev/null | grep -e '\* ' | sed 's/^..\(.*\)/\1/')
  set -l git_status (git status -s)
  echo $branch
end

function nv
    $EDITOR $argv
end

function ll
    exa -la --git --group-directories-first $argv
end

function l
    exa -l --git --group-directories-first $argv
end

function gc
    git commit $argv
end

function gst
    git status $argv
end

function gd
    git diff $argv
end

function gds
    git diff --staged $argv
end

function ga
    git add .
end

function ggp
    set -l branch (parse_git_branch)
    git push origin $branch
end

function ggpl
    set -l branch (parse_git_branch)
    git pull origin $branch
end

function glg
    git log --graph --show-signature
end

function gco
    git checkout $argv
end


function edit-fish-config
    $EDITOR $HOME/.config/fish/config.fish
end

function add-dir-to-path-front
    set PATH $PATH $argv
end

# Created by `userpath` on 2021-02-01 21:59:20
add-dir-to-path-front $HOME/.local/bin
add-dir-to-path-front $HOME/.npm-packages/bin
