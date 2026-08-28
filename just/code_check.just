alias lint := clippy

# Format all sources (Markdown, TOML, YAML and Rust via nightly rustfmt) with dprint
[group('code_check')]
fmt args="":
    dprint fmt {{ args }}

# Check formatting of all sources with dprint (no writes)
[group('code_check')]
fmt_check args="":
    dprint check {{ args }}

# Run clippy on all targets
[group('code_check')]
clippy args="":
    cargo clippy --workspace --all-targets {{ args }}

# Build the crate documentation, denying warnings
[group('code_check')]
doc args="":
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps {{ args }}

# Check dependencies for advisories, licenses, bans and sources (cargo-deny)
[group('code_check')]
deny args="":
    cargo deny check {{ args }}

# Scan for secrets with trufflehog (defaults to the whole working tree)
[group('code_check')]
scan_secrets *args=".":
    trufflehog filesystem {{ args }} --results=verified,unknown --fail --no-update

# Point git at the tracked .githooks directory (installs the pre-commit hook)
[group('code_check')]
setup_githooks:
    git config core.hooksPath .githooks
    @echo "git hooks installed: core.hooksPath = .githooks"

# Lint the install scripts (shellcheck; PowerShell parse when pwsh is available)
[group('code_check')]
check_install_scripts:
    sh -n install.sh
    shellcheck install.sh
    @if command -v pwsh >/dev/null 2>&1; then \
      pwsh -NoProfile -Command '$t = $null; $e = $null; $null = [System.Management.Automation.Language.Parser]::ParseFile("install.ps1", [ref]$t, [ref]$e); if ($e) { $e; exit 1 }'; \
    else \
      echo "pwsh not found: skipping install.ps1 parse check"; \
    fi

# Run all code checks. Fails if any check fails
[group('code_check')]
check_code:
    just fmt_check
    just clippy "-- -D warnings"
    just doc
    just deny
    just check_install_scripts
