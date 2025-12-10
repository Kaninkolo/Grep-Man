#!/bin/bash
# Bash completion script for gman
# Install: Copy to /etc/bash_completion.d/ or source in your ~/.bashrc

_gman_completions() {
    local cur prev words cword
    _init_completion || return

    # Available flags
    local flags="-c --case-sensitive -r --regex -h --help -V --version"

    # Determine which positional argument we're completing
    local positional_count=0
    local program=""
    local i
    for ((i = 1; i < cword; i++)); do
        case "${words[i]}" in
            -c|--case-sensitive|-r|--regex)
                # Flag without argument, skip
                ;;
            -*)
                # Unknown flag, skip
                ;;
            *)
                # Positional argument
                if [[ $positional_count -eq 0 ]]; then
                    program="${words[i]}"
                fi
                ((positional_count++))
                ;;
        esac
    done

    case $positional_count in
        0)
            # First positional argument: program name
            # If starts with -, complete gman flags
            if [[ ${cur} == -* ]]; then
                COMPREPLY=($(compgen -W "${flags}" -- "${cur}"))
            else
                # Complete with available man pages
                _gman_complete_programs
            fi
            ;;
        1)
            # Second positional argument: search term
            # Prioritize flags from the target program's man page
            if [[ -n "$program" && "$program" != -* ]]; then
                _gman_complete_flags_from_man "$program"
            fi
            # If no completions from man page, fall back to gman flags
            if [[ ${#COMPREPLY[@]} -eq 0 && ${cur} == -* ]]; then
                COMPREPLY=($(compgen -W "${flags}" -- "${cur}"))
            fi
            ;;
        *)
            # Additional arguments: only complete gman flags
            if [[ ${cur} == -* ]]; then
                COMPREPLY=($(compgen -W "${flags}" -- "${cur}"))
            fi
            ;;
    esac
}

_gman_complete_programs() {
    # Get list of available man pages
    # This tries multiple approaches for better coverage
    local programs=()

    # Method 1: Use apropos/man -k (most comprehensive but can be slow)
    if command -v apropos &>/dev/null; then
        programs+=($(apropos -s 1,8 . 2>/dev/null | cut -d' ' -f1 | cut -d'(' -f1))
    fi

    # Method 2: Common binaries from PATH (fast fallback)
    if [[ ${#programs[@]} -eq 0 ]]; then
        programs+=($(compgen -c))
    fi

    # Filter and complete
    COMPREPLY=($(compgen -W "${programs[*]}" -- "${cur}"))
}

_gman_complete_flags_from_man() {
    local program="$1"

    # Extract flags and subcommands from the man page
    if command -v man &>/dev/null; then
        local manpage=$(man -P cat "$program" 2>/dev/null)
        local items=""

        # Extract flags (starting with - or --)
        local flags=$(echo "$manpage" |
            grep -oE '(^|[[:space:],])(--?[a-zA-Z0-9][-a-zA-Z0-9_]*)' |
            sed 's/^[[:space:],]*//' |
            grep -E '^--?[a-zA-Z]' |
            sort -u)

        items="$flags"

        # Extract subcommands (e.g., git-add, git-commit)
        # Look for patterns like "program-subcommand"
        local subcommands=$(echo "$manpage" |
            grep -oE "${program}-[a-z][a-z0-9-]+" |
            sed "s/^${program}-//" |
            sort -u)

        items="$items $subcommands"

        if [[ -n "$items" ]]; then
            COMPREPLY=($(compgen -W "${items}" -- "${cur}"))
            return 0
        fi
    fi
    return 1
}

# Register the completion function
complete -F _gman_completions gman
