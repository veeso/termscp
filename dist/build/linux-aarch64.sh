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
ARM64_RPM_NAME="termscp-arm64_rpm"

set -e # Don't fail

# Create pkgs directory
cd ..
PKGS_DIR=$(pwd)/pkgs
cd -
mkdir -p ${PKGS_DIR}/
# Build aarch64_deb
cd aarch64_debian9/
docker buildx build --platform linux/arm64 $CACHE --build-arg branch=${BRANCH} --tag $ARM64_DEB_NAME .
cd -
mkdir -p ${PKGS_DIR}/deb/
mkdir -p ${PKGS_DIR}/aarch64-unknown-linux-gnu/
docker run --name "$ARM64_DEB_NAME" -d "$ARM64_DEB_NAME" || docker start "$ARM64_DEB_NAME"
docker exec -it "$ARM64_DEB_NAME" bash -c ". \$HOME/.cargo/env && git fetch origin && git checkout origin/$BRANCH && cargo build --release && cargo deb"
docker cp ${ARM64_DEB_NAME}:/usr/src/termscp/target/debian/termscp_${VERSION}_arm64.deb ${PKGS_DIR}/deb/
docker cp ${ARM64_DEB_NAME}:/usr/src/termscp/target/release/termscp ${PKGS_DIR}/aarch64-unknown-linux-gnu/
docker stop "$ARM64_DEB_NAME"
# Make tar.gz
cd ${PKGS_DIR}/aarch64-unknown-linux-gnu/
tar cvzf termscp-v${VERSION}-aarch64-unknown-linux-gnu.tar.gz termscp
rm termscp
cd -
# Build aarch64_centos7
cd aarch64_centos7/
docker buildx build --platform linux/arm64 $CACHE --build-arg branch=${BRANCH} --tag $ARM64_RPM_NAME .
cd -
mkdir -p ${PKGS_DIR}/rpm/
docker run --name "$ARM64_RPM_NAME" -d "$ARM64_RPM_NAME" || docker start "$ARM64_RPM_NAME"
docker exec -it "$ARM64_RPM_NAME" bash -c ". \$HOME/.cargo/env && git fetch origin && git checkout origin/$BRANCH; cargo rpm init; cargo rpm build"
docker cp ${ARM64_RPM_NAME}:/usr/src/termscp/target/release/rpmbuild/RPMS/aarch64/termscp-${VERSION}-1.el7.aarch64.rpm ${PKGS_DIR}/rpm/termscp-${VERSION}-1.aarch64.rpm
docker stop "$ARM64_RPM_NAME"

exit $?
