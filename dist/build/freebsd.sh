#!/bin/sh

if [ -z "$1" ]; then
    echo "Usage: freebsd.sh <version>"
    exit 1
fi

VERSION=$1

set -e # Don't fail

# Go to root dir
cd ../../
# Check if in correct directory
if [ ! -f Cargo.toml ]; then
    echo "Please start freebsd.sh from dist/build/ directory"
    exit 1
fi

# Build release
cargo build --release && cargo strip
# Make pkg
cd target/release/
PKG="termscp-v${VERSION}-x86_64-unknown-freebsd.tar.gz"
tar czf $PKG termscp
sha256sum $PKG
mkdir -p ../../dist/pkgs/freebsd/
mv $PKG ../../dist/pkgs/freebsd/$PKG

exit $?
