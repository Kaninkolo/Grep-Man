# gman

A CLI tool for searching man pages and jumping directly to specific lines. Built
with Rust and ratatui.

## Features

- Search for terms in man pages
- Interactive TUI for browsing search results
- Jump directly to the selected line in the man page
- Context display (shows line before match)
- Case-sensitive and case-insensitive search
- Bash/Zsh completion for program names and flags
- Vim-style navigation (j/k for up/down)

## Installation

### Build from source

```bash
cargo build --release
```

The binary will be at `target/release/gman`.

### Install system-wide

```bash
cargo install --path .
```

### Enable shell completion

#### Bash

```bash
# Copy the completion script
sudo cp gman-completion.bash /etc/bash_completion.d/gman

# Or source it in your ~/.bashrc
echo 'source /path/to/gman/gman-completion.bash' >> ~/.bashrc
source ~/.bashrc
```

#### Zsh

```bash
# Create completions directory if it doesn't exist
mkdir -p ~/.zsh/completions

# Copy the Zsh completion script
cp _gman ~/.zsh/completions/

# Add to your ~/.zshrc (if not already present)
echo 'fpath=(~/.zsh/completions $fpath)' >> ~/.zshrc
echo 'autoload -Uz compinit && compinit' >> ~/.zshrc

# Reload your shell
source ~/.zshrc
```

## Usage

```bash
gman [OPTIONS] <program> [search-term]
```

### Examples

```bash
# Open man page directly (no search)
gman ls

# Search for "recursive" in ls man page
gman ls recursive

# Case-sensitive search
gman -c grep pattern 

# Search for flags
gman tar extract
```

![example.png](example.png)

### Interactive Navigation

Once the search results appear:

- Use arrow keys or `j`/`k` to navigate
- Press `Enter` to jump to the selected line in the man page
- Press `q` or `Esc` to quit without jumping

### Options

- `-c, --case-sensitive` - Enable case-sensitive search (must be specified
  before the program name)
- `-h, --help` - Show help information (only works without a program name)
- `-V, --version` - Show version information (only works without a program name)

**Note:** Flags like `-h`, `--help`, etc. can be used as search terms! Once you
specify a program name, everything after is treated as a search term:

```bash
gman --help          # Shows gman help
gman git --help      # Searches for "--help" in git man page
gman git -h          # Searches for "-h" in git man page
```

## How It Works

When a search term is provided:

1. Extracts the man page text using `man -P cat`
2. Strips control characters used for formatting
3. Searches for the term and collects matching lines with context
4. Displays an interactive TUI for selection
5. Opens the man page at the selected line using `less +<line>G`

When no search term is provided:

- Opens the man page directly

## Shell Completion

The completion scripts provide:

- Program name completion (completes from available man pages)
- Flag/parameter suggestions from the target program's man page
- Flag completion for gman's own flags (`-c`, `--case-sensitive`, etc.)

Example workflow:

```bash
gman <TAB>          # Shows available programs (ls, grep, tar, etc.)
gman ls <TAB>       # Shows flags from ls man page (-a, -l, --all, etc.)
gman ls -a<TAB>     # Filters to flags starting with -a
gman --c<TAB>       # Completes gman's own flag to --case-sensitive
```

## Requirements

- Rust 1.70 or later
- `man` command (standard on Unix-like systems, except Arch)
- `less` pager (standard on Unix-like systems)

## License

MIT
