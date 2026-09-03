#!/usr/bin/env sh
# Builds a static musl termscp release binary for the given target triple
# inside a pinned Alpine container.
#
# Usage: dist/release/build_musl.sh <target-triple>
set -eu

IMAGE="rust:1.98-alpine3.22"

TARGET="${1:-}"
if [ -z "$TARGET" ]; then
  echo "usage: $0 <target-triple>" >&2
  exit 2
fi

case "$TARGET" in
  x86_64-unknown-linux-musl) PLATFORM="linux/amd64" ;;
  aarch64-unknown-linux-musl) PLATFORM="linux/arm64" ;;
  *)
    echo "unsupported target: $TARGET" >&2
    exit 2
    ;;
esac

WORKSPACE="$(CDPATH='' cd -- "$(dirname -- "$0")/../.." && pwd)"

# The container appends a [patch.crates-io] section to Cargo.toml and updates
# Cargo.lock; keep byte-exact copies so the workspace is clean afterwards.
BACKUP_DIR="$(mktemp -d)"
cp -p "$WORKSPACE/Cargo.toml" "$BACKUP_DIR/Cargo.toml"
cp -p "$WORKSPACE/Cargo.lock" "$BACKUP_DIR/Cargo.lock"

restore_manifests() {
  cp -p "$BACKUP_DIR/Cargo.toml" "$WORKSPACE/Cargo.toml"
  cp -p "$BACKUP_DIR/Cargo.lock" "$WORKSPACE/Cargo.lock"
  rm -rf "$BACKUP_DIR"
}
trap restore_manifests EXIT

HOST_UID="$(id -u)"
HOST_GID="$(id -g)"
export TARGET HOST_UID HOST_GID

docker run --rm \
  --platform "$PLATFORM" \
  --env TARGET \
  --env HOST_UID \
  --env HOST_GID \
  --volume "$WORKSPACE:/work" \
  --workdir /work \
  "$IMAGE" \
  sh /work/dist/release/build_musl_container.sh
