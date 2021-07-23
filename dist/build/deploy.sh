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
cd x86_64_debian9/
docker build --tag termscp-${VERSION}-x86_64_debian9 .
cd -
mkdir -p ${PKGS_DIR}/deb/
CONTAINER_NAME=$(docker create termscp-${VERSION}-x86_64_debian9 termscp-${VERSION}-x86_64_debian9)
docker cp ${CONTAINER_NAME}:/usr/src/termscp/target/debian/termscp_${VERSION}_amd64.deb ${PKGS_DIR}/deb/
# Build x86_64_centos7
cd x86_64_centos7/
docker build --tag termscp-${VERSION}-x86_64_centos7 .
cd -
mkdir -p ${PKGS_DIR}/rpm/
CONTAINER_NAME=$(docker create termscp-${VERSION}-x86_64_centos7 termscp-${VERSION}-x86_64_centos7)
docker cp ${CONTAINER_NAME}:/usr/src/termscp/target/release/rpmbuild/RPMS/x86_64/termscp-${VERSION}-1.el7.x86_64.rpm ${PKGS_DIR}/rpm/termscp-${VERSION}-1.x86_64.rpm

exit $?
