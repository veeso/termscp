#!/usr/bin/env bash
# Bump termscp version across all tracked locations.
# Usage: bump_version.sh <version> [date] [root]
set -euo pipefail

VERSION="${1:?usage: bump_version.sh <version> [date] [root]}"
DATE="${2:-$(date +%F)}"
ROOT="${3:-$(git rev-parse --show-toplevel)}"

# in-place sed that works on both GNU and BSD/macOS
sedi() { perl -0777 -pi -e "$1" "$2"; }

# Cargo.toml — only the top-level package version (line-anchored), not deps
sedi "s/^version = \"[0-9][0-9A-Za-z.\\-]*\"/version = \"$VERSION\"/m" "$ROOT/Cargo.toml"

# install.sh — the literal default assignment only
sedi "s/^TERMSCP_VERSION=\"[0-9][0-9A-Za-z.\\-]*\"/TERMSCP_VERSION=\"$VERSION\"/m" "$ROOT/install.sh"

# install.ps1 — the default -Version parameter value only
sedi "s/\\\$Version = \"[0-9][0-9A-Za-z.\\-]*\"/\\\$Version = \"$VERSION\"/" "$ROOT/install.ps1"

# README.md — version + release date
sedi "s/Current version: [0-9][0-9A-Za-z.\\-]* [0-9]{4}-[0-9]{2}-[0-9]{2}/Current version: $VERSION $DATE/" "$ROOT/README.md"

# site: version constant displayed on the website
sedi "s/^export const VERSION = \"[0-9][0-9A-Za-z.\\-]*\";/export const VERSION = \"$VERSION\";/m" "$ROOT/site/src/consts.ts"

# chocolatey nuspec
sedi "s#<version>[0-9][0-9A-Za-z.\\-]*</version>#<version>$VERSION</version>#" "$ROOT/dist/chocolatey/termscp.nuspec"

# chocolatey install script — release tag + asset name in the URLs (checksums set later by CI)
sedi "s#releases/download/v[0-9][0-9A-Za-z.\\-]*/termscp-v[0-9][0-9A-Za-z.\\-]*-#releases/download/v$VERSION/termscp-v$VERSION-#g" "$ROOT/dist/chocolatey/tools/chocolateyinstall.ps1"

echo "Bumped to $VERSION ($DATE)"
