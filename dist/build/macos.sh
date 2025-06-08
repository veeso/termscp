#!/bin/sh

make_pkg() {
    ARCH=$1
    VERSION=$2
    TARGET_DIR="$3"
    if [ -z "$TARGET_DIR" ]; then
        TARGET_DIR=target/release/
    fi
    ROOT_DIR=$(pwd)
    cd "$TARGET_DIR"
    PKG="termscp-v${VERSION}-${ARCH}-apple-darwin.tar.gz"
    tar czf "$PKG" termscp
    HASH=$(sha256sum "$PKG")
    mkdir -p "${ROOT_DIR}/dist/pkgs/macos/"
    mv "$PKG" "${ROOT_DIR}/dist/pkgs/macos/$PKG"
    cd -
    echo "$HASH"
}

detect_platform() {
    local platform
    platform="$(uname -s | tr '[:upper:]' '[:lower:]')"

    case "${platform}" in
        linux) platform="linux" ;;
        darwin) platform="macos" ;;
        freebsd) platform="freebsd" ;;
    esac

    printf '%s' "${platform}"
}

detect_arch() {
    local arch
    arch="$(uname -m | tr '[:upper:]' '[:lower:]')"

    case "${arch}" in
        amd64) arch="x86_64" ;;
        armv*) arch="arm" ;;
        arm64) arch="aarch64" ;;
    esac

    # `uname -m` in some cases mis-reports 32-bit OS as 64-bit, so double check
    if [ "${arch}" = "x86_64" ] && [ "$(getconf LONG_BIT)" -eq 32 ]; then
        arch="i686"
    elif [ "${arch}" = "aarch64" ] && [ "$(getconf LONG_BIT)" -eq 32 ]; then
        arch="arm"
    fi

    printf '%s' "${arch}"
}

if [ -z "$1" ]; then
    echo "Usage: macos.sh <version>"
    exit 1
fi

PLATFORM="$(detect_platform)"
ARCH="$(detect_arch)"

if [ "$PLATFORM" != "macos" ]; then
    echo "macos build is only available on MacOs systems"
    exit 1
fi

VERSION=$1
export BUILD_ROOT
BUILD_ROOT="$(pwd)/../../"

set -e # Don't fail

# Go to root dir
cd ../../
# Check if in correct directory
if [ ! -f Cargo.toml ]; then
    echo "Please start macos.sh from dist/build/ directory"
    exit 1
fi

# Build release (x86_64)
X86_TARGET=""
X86_TARGET_DIR=""
if [ "$ARCH" = "x86_64" ]; then
    X86_TARGET="--target x86_64-apple-darwin"
    X86_TARGET_DIR="target/x86_64-apple-darwin/release/"
fi
cargo build --release $X86_TARGET
# Make pkg
X86_64_HASH=$(make_pkg "x86_64" "$VERSION" $X86_TARGET_DIR)
RET_X86_64=$?

ARM64_TARGET=""
ARM64_TARGET_DIR=""
if [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
    ARM64_TARGET="--target aarch64-apple-darwin"
    ARM64_TARGET_DIR="target/aarch64-apple-darwin/release/"
fi
cd "$BUILD_ROOT"
# Build ARM64 pkg
cargo build --release $ARM64_TARGET
# Make pkg
ARM64_HASH=$(make_pkg "arm64" "$VERSION" $ARM64_TARGET_DIR)
RET_ARM64=$?

echo "x86_64 hash: $X86_64_HASH"
echo "arm64  hash: $ARM64_HASH"

[ "$RET_ARM64" -eq 0 ] && [ "$RET_X86_64" -eq 0 ]
exit $?
