#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: deploy.sh <version>"
    exit 1
fi

VERSION=$1

set -e # Don't fail

# Create pkgs directory
cd ..
PKGS_DIR=$(pwd)/pkgs
cd -
mkdir -p ${PKGS_DIR}/
# Build x86_64_deb
cd x86_64_debian8/
docker build --tag termscp-${VERSION}-x86_64_debian8 .
cd -
mkdir -p ${PKGS_DIR}/deb/
CONTAINER_NAME=$(docker create termscp-${VERSION}-x86_64_debian8 termscp-${VERSION}-x86_64_debian8)
docker cp ${CONTAINER_NAME}:/usr/src/termscp/target/debian/termscp_${VERSION}_amd64.deb ${PKGS_DIR}/deb/
# Build x86_64_centos7
cd x86_64_centos7/
docker build --tag termscp-${VERSION}-x86_64_centos7 .
cd -
mkdir -p ${PKGS_DIR}/rpm/
CONTAINER_NAME=$(docker create termscp-${VERSION}-x86_64_centos7 termscp-${VERSION}-x86_64_centos7)
docker cp ${CONTAINER_NAME}:/usr/src/termscp/target/release/rpmbuild/RPMS/x86_64/termscp-${VERSION}-1.x86_64.rpm ${PKGS_DIR}/rpm/
# Build x86_64_archlinux

##################### TEMP REMOVED  ###################################
# cd x86_64_archlinux/
# docker build --tag termscp-${VERSION}-x86_64_archlinux .
# # Create container and get AUR pkg
# cd -
# mkdir -p ${PKGS_DIR}/arch/
# CONTAINER_NAME=$(docker create termscp-${VERSION}-x86_64_archlinux termscp-${VERSION}-x86_64_archlinux)
# docker cp ${CONTAINER_NAME}:/usr/src/termscp/termscp-${VERSION}-x86_64.tar.gz ${PKGS_DIR}/arch/
# docker cp ${CONTAINER_NAME}:/usr/src/termscp/PKGBUILD ${PKGS_DIR}/arch/
# docker cp ${CONTAINER_NAME}:/usr/src/termscp/.SRCINFO ${PKGS_DIR}/arch/
# # Replace termscp-bin with termscp in PKGBUILD
# sed -i 's/termscp-bin/termscp/g' ${PKGS_DIR}/arch/PKGBUILD
##################### TEMP REMOVED  ###################################

exit $?
