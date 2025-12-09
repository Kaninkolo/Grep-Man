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
gman <program> <search-term> [OPTIONS]
```

### Examples

```bash
# Search for "recursive" in ls man page
gman ls recursive

# Case-sensitive search
gman grep pattern -c

# Search for flags
gman tar extract
```

### Interactive Navigation

Once the search results appear:

- Use arrow keys or `j`/`k` to navigate
- Press `Enter` to jump to the selected line in the man page
- Press `q` or `Esc` to quit without jumping

### Options

- `-c, --case-sensitive` - Enable case-sensitive search (default:
  case-insensitive)
- `-h, --help` - Show help information
- `-V, --version` - Show version

## How It Works

1. Extracts the man page text using `man -P cat`
2. Strips control characters used for formatting
3. Searches for the term and collects matching lines with context
4. Displays an interactive TUI for selection
5. Opens the man page at the selected line using `less +<line>G`

## Bash Completion

The completion script provides:

- Program name completion (completes from available man pages)
- Flag completion from the target program's man page
- gman's own flags completion

Example workflow:

```bash
gman <TAB>          # Shows available programs
gman ls <TAB>       # Shows flags from ls man page
gman ls -a<TAB>     # Completes flags starting with -a
```

## Requirements

- Rust 1.70 or later
- `man` command (standard on Unix-like systems)
- `less` pager (standard on Unix-like systems)

## License

MIT
