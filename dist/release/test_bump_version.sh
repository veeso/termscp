#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BUMP="$SCRIPT_DIR/bump_version.sh"

ROOT="$(mktemp -d)"
trap 'rm -rf "$ROOT"' EXIT

# --- build a fixture tree mirroring the real layout, all at 1.0.0 ---
mkdir -p "$ROOT/site/html" "$ROOT/site/lang" "$ROOT/dist/chocolatey/tools"

cat > "$ROOT/Cargo.toml" <<'EOF'
[package]
name = "termscp"
version = "1.0.0"

[dependencies]
foo = { version = "1.0.0" }
EOF

cat > "$ROOT/install.sh" <<'EOF'
TERMSCP_VERSION="1.0.0"
set_termscp_version() {
    TERMSCP_VERSION="$1"
}
EOF

cat > "$ROOT/README.md" <<'EOF'
<p align="center">Current version: 1.0.0 2026-04-18</p>
EOF

cat > "$ROOT/site/html/home.html" <<'EOF'
<span>termscp 1.0.0 is NOW out! Download it from</span>
EOF

cat > "$ROOT/site/html/get-started.html" <<'EOF'
<a href="https://github.com/veeso/termscp/releases/latest/download/termscp.1.0.0.nupkg">Github</a>
<pre>wget -O termscp.deb https://github.com/veeso/termscp/releases/latest/download/termscp_1.0.0_amd64.deb</pre>
EOF

for lang in en it fr es zh-CN; do
  cat > "$ROOT/site/lang/$lang.json" <<'EOF'
{ "versionAlert": "termscp 1.0.0 is NOW out! Download it from" }
EOF
done

cat > "$ROOT/dist/chocolatey/termscp.nuspec" <<'EOF'
<version>1.0.0</version>
EOF

cat > "$ROOT/dist/chocolatey/tools/chocolateyinstall.ps1" <<'EOF'
$url = 'https://github.com/veeso/termscp/releases/download/v1.0.0/termscp-v1.0.0-aarch64-pc-windows-msvc.zip'
$url = 'https://github.com/veeso/termscp/releases/download/v1.0.0/termscp-v1.0.0-x86_64-pc-windows-msvc.zip'
EOF

# --- run the bump ---
"$BUMP" 1.1.0 2026-06-07 "$ROOT"

fail() { echo "FAIL: $1"; exit 1; }
have() { grep -q -- "$2" "$ROOT/$1" || fail "$1 missing: $2"; }
missing() { ! grep -q -- "$2" "$ROOT/$1" || fail "$1 still has: $2"; }

# package version bumped, dependency version NOT touched
have  "Cargo.toml" 'version = "1.1.0"'
have  "Cargo.toml" 'foo = { version = "1.0.0" }'

have  "install.sh" 'TERMSCP_VERSION="1.1.0"'
have  "README.md" 'Current version: 1.1.0 2026-06-07'
missing "README.md" '2026-04-18'

have  "site/html/home.html" 'termscp 1.1.0 is NOW out'
have  "site/lang/en.json" 'termscp 1.1.0 is NOW out'
have  "site/lang/zh-CN.json" 'termscp 1.1.0 is NOW out'
have  "site/html/get-started.html" 'termscp.1.1.0.nupkg'
have  "site/html/get-started.html" 'termscp_1.1.0_amd64.deb'
have  "dist/chocolatey/termscp.nuspec" '<version>1.1.0</version>'
have  "dist/chocolatey/tools/chocolateyinstall.ps1" 'releases/download/v1.1.0/termscp-v1.1.0-'
missing "dist/chocolatey/tools/chocolateyinstall.ps1" 'v1.0.0'

# --- idempotency: running again at same version is a no-op (no error) ---
"$BUMP" 1.1.0 2026-06-07 "$ROOT"
have "Cargo.toml" 'version = "1.1.0"'

echo "PASS"
