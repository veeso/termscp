#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <version> [branch] [--no-cache]"
    exit 1
fi

VERSION=$1

if [ -z "$2" ]; then
    BRANCH=$VERSION
else
    BRANCH=$2
fi

CACHE=""

if [ "$3" == "--no-cache" ]; then
    CACHE="--no-cache"
fi

# names
ARM64_DEB_NAME="termscp-arm64_deb"

docker run --rm --privileged multiarch/qemu-user-static --reset -p yes

set -e # Don't fail

# Create pkgs directory
cd ..
PKGS_DIR=$(pwd)/pkgs
cd -
mkdir -p ${PKGS_DIR}/
# Build aarch64_deb
cd aarch64_debian10/
docker buildx build --platform linux/arm64 $CACHE --build-arg branch=${BRANCH} --tag $ARM64_DEB_NAME .
cd -
mkdir -p ${PKGS_DIR}/deb/
mkdir -p ${PKGS_DIR}/aarch64-unknown-linux-gnu/
docker run --name "$ARM64_DEB_NAME" -d "$ARM64_DEB_NAME" || docker start "$ARM64_DEB_NAME"
docker exec -it "$ARM64_DEB_NAME" bash -c ". \$HOME/.cargo/env && git fetch origin && git checkout origin/$BRANCH && cargo build --release --features smb-vendored && cargo deb"
docker cp ${ARM64_DEB_NAME}:/usr/src/termscp/target/debian/termscp_${VERSION}-1_arm64.deb ${PKGS_DIR}/deb/termscp_${VERSION}_arm64.deb
docker cp ${ARM64_DEB_NAME}:/usr/src/termscp/target/release/termscp ${PKGS_DIR}/aarch64-unknown-linux-gnu/
docker stop "$ARM64_DEB_NAME"
# Make tar.gz
cd ${PKGS_DIR}/aarch64-unknown-linux-gnu/
tar cvzf termscp-v${VERSION}-aarch64-unknown-linux-gnu.tar.gz termscp
echo "Sha256 (homebrew aarch64): $(sha256sum termscp-v${VERSION}-aarch64-unknown-linux-gnu.tar.gz)"
rm termscp
cd -

exit $?
