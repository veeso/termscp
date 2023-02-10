#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

VERSION=$1

set -e # Don't fail

# Create pkgs directory
cd ..
PKGS_DIR=$(pwd)/pkgs
cd -
mkdir -p ${PKGS_DIR}/
# Build aarch64_deb
cd aarch64_debian9/
docker buildx build --platform linux/arm64 --build-arg branch=${VERSION} --tag termscp-${VERSION}-aarch64_debian9 .
cd -
mkdir -p ${PKGS_DIR}/deb/
mkdir -p ${PKGS_DIR}/aarch64-unknown-linux-gnu/
CONTAINER_NAME=$(docker create termscp-${VERSION}-aarch64_debian9 /bin/bash)
docker cp ${CONTAINER_NAME}:/usr/src/termscp/target/debian/termscp_${VERSION}_arm64.deb ${PKGS_DIR}/deb/
docker cp ${CONTAINER_NAME}:/usr/src/termscp/target/release/termscp ${PKGS_DIR}/aarch64-unknown-linux-gnu/
# Make tar.gz
cd ${PKGS_DIR}/aarch64-unknown-linux-gnu/
tar cvzf termscp-v${VERSION}-aarch64-unknown-linux-gnu.tar.gz termscp
rm termscp
cd -
# Build aarch64_centos7
cd aarch64_centos7/
docker buildx build --platform linux/arm64 --build-arg branch=${VERSION} --tag termscp-${VERSION}-aarch64_centos7 .
cd -
mkdir -p ${PKGS_DIR}/rpm/
CONTAINER_NAME=$(docker create termscp-${VERSION}-aarch64_centos7 /bin/bash)
docker cp ${CONTAINER_NAME}:/usr/src/termscp/target/release/rpmbuild/RPMS/aarch64/termscp-${VERSION}-1.el7.arm64.rpm ${PKGS_DIR}/rpm/termscp-${VERSION}-1.arm64.rpm

exit $?
