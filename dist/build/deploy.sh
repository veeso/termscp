#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: deploy.sh <version>"
    exit 1
fi

VERSION=$1

mkdir -p pkgs/${VERSION}/
# Build x86_64
cd x86_64/
docker build --tag termscp-${VERSION}-x86_64 .
# Create container and get deb, rpm
cd -
mkdir -p pkgs/${VERSION}/deb/
mkdir -p pkgs/${VERSION}/rpm/
CONTAINER_NAME=$(docker create termscp-${VERSION}-x86_64 termscp-${VERSION}-x86_64)
docker cp ${CONTAINER_NAME}:/usr/src/termscp/target/debian/termscp_${VERSION}_amd64.deb pkgs/${VERSION}/deb/
docker cp ${CONTAINER_NAME}:/usr/src/termscp/target/release/rpmbuild/RPMS/x86_64/termscp-${VERSION}-1.x86_64.rpm pkgs/${VERSION}/rpm/
# Build x86_64_archlinux
cd x86_64_archlinux/
docker build --tag termscp-${VERSION}-x86_64_archlinux .
# Create container and get AUR pkg
cd -
mkdir -p pkgs/${VERSION}/arch/
CONTAINER_NAME=$(docker create termscp-${VERSION}-x86_64_archlinux termscp-${VERSION}-x86_64_archlinux)
docker cp ${CONTAINER_NAME}:/usr/src/termscp/termscp-${VERSION}-x86_64.tar.gz pkgs/arch/
docker cp ${CONTAINER_NAME}:/usr/src/termscp/PKGBUILD pkgs/${VERSION}/arch/

exit $?
