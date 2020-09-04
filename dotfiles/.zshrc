ZSH=$HOME/.oh-my-zsh
ZSH_THEME="blinks"
DISABLE_AUTO_UPDATE="true"

plugins=(git rbates)

alias nv=nvim
alias vim='echo "use nvim === nv instead"'

export EDITOR='vim'

source $ZSH/oh-my-zsh.sh

export PATH="$HOME/vpn/bin:$PATH"

export PATH="/usr/local/bin:$PATH"
export PATH="$HOME/bin:$PATH"

export GOPATH="$HOME/go"
export PATH="$GOPATH/bin:$PATH"
export PATH="/usr/local/go/bin:$PATH"

export JAVA_HOME="/usr/lib/jvm/java-8-oracle"

export RUST_BACKTRACE=full
export RUSTUP_HOME="$HOME/.rustup"
export PATH="$HOME/.cargo/bin:$PATH"
export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"

if [ -d "$HOME/bin" ]; then
   PATH="$HOME/bin:$PATH"
fi

if [[ -r /usr/local/lib/python2.7/site-packages/powerline/bindings/zsh/powerline.zsh ]]; then
      source /usr/local/lib/python2.7/site-packages/powerline/bindings/zsh/powerline.zsh
fi
