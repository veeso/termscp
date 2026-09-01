[group('run')]
run *args:
    @set -m; cargo run --bin termscp -- {{ args }}

# Build, sign, and run the macOS CLI binary with a stable development identity
# Job control gives smista its own foreground process group, so Ctrl-C reaches it
# without also interrupting just.
[group('run')]
run_signed *args:
    @test "$(uname -s)" = "Darwin" || (echo "run_signed is only supported on macOS." >&2; exit 1)
    cargo build --bin termscp
    codesign --force --sign "${TERMSCP_CODESIGN_IDENTITY:-Apple Development}" target/debug/termscp
    @set -m; target/debug/termscp {{ args }}
