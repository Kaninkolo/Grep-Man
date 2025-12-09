#!/bin/bash
# Bash completion script for gman
# Install: Copy to /etc/bash_completion.d/ or source in your ~/.bashrc

_gman_completions() {
    local cur prev words cword
    _init_completion || return

    # Available flags
    local flags="-c --case-sensitive -h --help -V --version"

    # If current word starts with -, complete flags
    if [[ ${cur} == -* ]]; then
        COMPREPLY=($(compgen -W "${flags}" -- "${cur}"))
        return 0
    fi

    # Determine which positional argument we're completing
    local positional_count=0
    local i
    for ((i = 1; i < cword; i++)); do
        case "${words[i]}" in
            -c|--case-sensitive)
                # Flag without argument, skip
                ;;
            -*)
                # Unknown flag, skip
                ;;
            *)
                # Positional argument
                ((positional_count++))
                ;;
        esac
    done

    case $positional_count in
        0)
            # First positional argument: program name
            # Complete with available man pages
            _gman_complete_programs
            ;;
        1)
            # Second positional argument: search term
            # If a program was specified, try to extract flags from its man page
            local program="${words[1]}"
            if [[ -n "$program" && "$program" != -* ]]; then
                _gman_complete_flags_from_man "$program"
            fi
            ;;
        *)
            # No completion for additional arguments
            return 0
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

    # Try to extract flags from the man page for the given program
    # This is a heuristic approach - looks for lines starting with - or --
    if command -v man &>/dev/null; then
        local flags=$(man -P cat "$program" 2>/dev/null |
            grep -oE '^\s*(--?[a-zA-Z0-9][-a-zA-Z0-9_]*)' |
            tr -d ' ' |
            sort -u |
            head -n 50)

        if [[ -n "$flags" ]]; then
            COMPREPLY=($(compgen -W "${flags}" -- "${cur}"))
        fi
    fi
}

# Register the completion function
complete -F _gman_completions gman
