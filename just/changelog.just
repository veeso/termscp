set positional-arguments

# Print the unreleased changelog section to stdout (preview only), e.g. `just changelog_preview 8.1.0`
[group('changelog')]
changelog_preview version:
    git-cliff --config cliff.toml --unreleased --tag "v$1" --strip all

# Add a new entry to CHANGELOG.md from the unreleased conventional commits, e.g. `just changelog 8.1.0`
[group('changelog')]
changelog version:
    #!/usr/bin/env bash
    set -euo pipefail
    version="$1"
    if [[ ! "$version" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]]; then
        echo "invalid release version: $version (expected MAJOR.MINOR.PATCH)" >&2
        exit 2
    fi
    tag="v$version"
    section="$(git-cliff --config cliff.toml --unreleased --tag "$tag" --strip all)"
    if [ -z "$section" ]; then
        echo "No unreleased conventional commits found; nothing to add." >&2
        exit 0
    fi
    anchor="$(printf '%s' "$version" | tr -d '.')"
    secfile="$(mktemp)"
    tmp="$(mktemp)"
    printf '%s\n' "$section" > "$secfile"
    # Insert a TOC entry before the first existing version entry and the rendered
    # section before the first existing version heading, keeping the title + TOC.
    awk -v secfile="$secfile" -v ver="$version" -v anc="$anchor" '
        !toc_done && /^[[:space:]]*- \[[0-9]/ {
            print "    - [" ver "](#" anc ")"
            toc_done = 1
        }
        !body_done && /^## [0-9]/ {
            while ((getline line < secfile) > 0) print line
            print ""
            body_done = 1
        }
        { print }
    ' CHANGELOG.md > "$tmp"
    rm -f "$secfile"
    mv "$tmp" CHANGELOG.md
    echo "CHANGELOG.md updated for $tag"
    just fmt CHANGELOG.md
