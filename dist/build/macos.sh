#!/bin/sh

make_pkg() {
    ARCH=$1
    VERSION=$2
    TARGET_DIR="$3"
    if [ -z "$TARGET_DIR" ]; then
        TARGET_DIR=target/release/
    fi
    ROOT_DIR=$(pwd)
    cd $TARGET_DIR
    PKG="termscp-v${VERSION}-${ARCH}-apple-darwin.tar.gz"
    tar czf $PKG termscp
    HASH=$(sha256sum $PKG)
    mkdir -p ${ROOT_DIR}/dist/pkgs/macos/
    mv $PKG ${ROOT_DIR}/dist/pkgs/macos/$PKG
    cd -
    echo "$HASH"
}

build_openssl_arm64() {
    # setup dirs
    BUILD_DIR=$(pwd)
    OPENSSL_BUILD_DIR=/tmp/openssl-build/
    # setup openssl dir
    mkdir -p $OPENSSL_DIR
    cd $OPENSSL_DIR
    # check if openssl has already been compiled
    if [ -e ./include/ ]; then
        return 0
    fi
    # download package
    TEMP_TGZ=/tmp/openssl.tar.gz
    wget https://www.openssl.org/source/openssl-1.1.1m.tar.gz -O $TEMP_TGZ
    # setup build dir
    mkdir -p $OPENSSL_BUILD_DIR
    cd $OPENSSL_BUILD_DIR
    # extract sources
    tar xzvf $TEMP_TGZ
    rm $TEMP_TGZ
    # build
    cd openssl-1.1.1m/
    export MACOSX_DEPLOYMENT_TARGET=10.15
    ./Configure enable-rc5 zlib darwin64-arm64-cc no-asm
    make
    make install DESTDIR=$(pwd)/out/
    # copy compiled assets to openssl dir
    cp -r out/usr/local/* $OPENSSL_DIR/
    # go back to build dir
    cd $BUILD_DIR
    # delete temp dir
    rm -rf $OPENSSL_BUILD_DIR
    return 0
}

if [ -z "$1" ]; then
    echo "Usage: macos.sh <version>"
    exit 1
fi

VERSION=$1
export BUILD_ROOT=$(pwd)/../../

set -e # Don't fail

# Go to root dir
cd ../../
# Check if in correct directory
if [ ! -f Cargo.toml ]; then
    echo "Please start macos.sh from dist/build/ directory"
    exit 1
fi

# Build release (x86_64)
cargo build --release
# Make pkg
X86_64_HASH=$(make_pkg "x86_64" $VERSION)

# set openssl dir
export OPENSSL_DIR=$BUILD_ROOT/dist/build/macos/openssl/
export OPENSSL_STATIC=1
export OPENSSL_LIB_DIR=${OPENSSL_DIR}/lib/
export OPENSSL_INCLUDE_DIR=${OPENSSL_DIR}/include/
# build openssl
build_openssl_arm64
cd $BUILD_ROOT
# Build ARM64 pkg
cargo build --release --target aarch64-apple-darwin
# Make pkg
ARM64_HASH=$(make_pkg "arm64" $VERSION "target/aarch64-apple-darwin/release/")

echo "x86_64 hash: $X86_64_HASH"
echo "arm64  hash: $ARM64_HASH"

exit $?
