" based on http://github.com/jferris/config_files/blob/master/vimrc

" Use Vim settings, rather then Vi settings (much better!).
" This must be first, because it changes other options as a side effect.
set nocompatible
filetype off

call plug#begin()

    Plug 'aklt/plantuml-syntax'

    Plug 'dag/vim-fish'
    Plug 'mhinz/vim-crates'
    Plug 'tpope/vim-surround'
    Plug 'tpope/vim-dispatch'
    Plug 'suan/vim-instant-markdown'
    Plug 'ekalinin/Dockerfile.vim'
    Plug 'flazz/vim-colorschemes'
    Plug 'w0rp/ale'
    Plug 'udalov/kotlin-vim'
    Plug 'ntpeters/vim-better-whitespace'
    Plug 'xolox/vim-colorscheme-switcher'
    Plug 'xolox/vim-reload'
    Plug 'xolox/vim-misc'
    Plug 'rust-lang/rust.vim'

    if has('nvim')
        Plug 'neovim/nvim-lsp'
    endif

call plug#end()

if has('nvim')
    lua require'nvim_lsp'.rust_analyzer.setup{}
    autocmd BufRead Cargo.toml call crates#toggle()
endif

filetype plugin indent on


" allow backspacing over everything in insert mode
set backspace=indent,eol,start

set nobackup
set nowritebackup
set history=50		" keep 50 lines of command line history
set ruler		" show the cursor position all the time
set showcmd		" display incomplete commands
set incsearch		" do incremental searching
set showbreak=... "set visual cue for linebreak

" Switch syntax highlighting on, when the terminal has colors
" Also switch on highlighting the last used search pattern.
if (&t_Co > 2 || has("gui_running")) && !exists("syntax_on")
	syntax on
	set hlsearch
endif

" netrw
let g:netrw_banner = 0
let g:netrw_winsize = -28
let g:netrw_liststyle = 3 " tree mode

"ale
let g:ale_enabled = 1
let g:ale_rust_cargo_use_clippy = 1

let g:ale_linters = {'rust': ['cargo']}
let g:ale_lint_on_save = 1

let g:ale_fixers = {'rust': ['rustfmt']}
let g:ale_fix_on_save = 1

let g:ale_completion_enabled = 1
let g:ale_sign_column_always = 1
let g:ale_sign_error = 'X'
let g:ale_sign_warning = 'W'
let g:ale_echo_msg_format = '[%linter%] %s [%severity%]'
let g:ale_open_list = 1

let g:rustfmt_autosave = 0

" YCM
let g:ycm_confirm_extra_conf = 0
let g:ycm_rust_src_path = $RUST_SRC_PATH

let g:strip_whitespace_on_save=1
let g:test_mode = 'test'

" Softtabs, 4 spaces
set tabstop=4
set shiftwidth=4
set expandtab
set cursorline
set cursorcolumn

" Always display the status line
set laststatus=2

let mapleader=","

function! SaveAndRunCargo(cmd)
    execute ':w'
    let c = ':!cargo ' . a:cmd
    execute c
endfunction

function! SaveAndRunGoTest(cmd)
    execute ':w'
    let c = ':!go test ' . a:cmd
    execute c
endfunction

let g:go_fmt_command = "goimports"
let g:golang_goroot = '/home/dan/Development'


noremap <Leader>f :Lexplore<CR> " toggle netrw pane
noremap <silent> <leader>y "+y<CR> " xdg clipboard yank
noremap <silent> <leader>p "+p<CR> " xdg clipboard plunk

noremap <Leader>r :call SaveAndRunCargo('run')<CR> " :RustRun doesn't work well with rustup
noremap <Leader>T :RustTest!<CR>
noremap <Leader>t :RustTest<CR>
noremap <Leader>b :call SaveAndRunCargo('build')<CR>
noremap <Leader>m :call SaveAndRunGoTest('./...')<CR>

noremap <silent> <leader>g :YcmCompleter GoToDefinition<CR>
noremap <leader>c :NextColorScheme<CR>
noremap <silent> <leader>s :wq<CR>
noremap <silent> <leader>w :w<CR>
noremap <silent> <leader>q :q<CR>

" Press Shift+P while in visual mode to replace the selection without
" overwriting the default register
vmap P p :call setreg('"', getreg('0')) <CR>

" Maps autocomplete to tab
imap <Tab> <C-N>

" Color scheme
if $TERM =~ '256color'
	set t_Co=256
elseif $TERM =~ '^xterm$'
	set t_Co=256
endif
syntax enable

" dark
"colorscheme badwolf
"colorscheme basic-dark
"colorscheme blacklight
"colorscheme brogrammer
"colorscheme cake16
"colorscheme coldgreen
"colorscheme colorful256
"colorscheme crt
"colorscheme elda
"colorscheme calmar256-dark
"colorscheme Atelier_ForestDark


" light
" colorscheme bubblegum-256-light
" colorscheme baycomb
colorscheme buddy


highlight NonText guibg=#060606
highlight Folded  guibg=#0A0A0A guifg=#9090D0

" Numbers
set number
set numberwidth=5

" Snippets are activated by Shift+Tab
let g:snippetsEmu_key = "<S-Tab>"

" Tab completion options
" (only complete to the longest unambiguous match, and show a menu)
set completeopt=longest,menu
set wildmode=list:longest,list:full
set complete=.,t

" case only matters with mixed case expressions
set ignorecase
set smartcase

" Tags
let g:Tlist_Ctags_Cmd="ctags --exclude='*.js'"
set tags=./tags;

let g:fuf_splitPathMatching=1
