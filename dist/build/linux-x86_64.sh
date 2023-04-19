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
X86_64_DEB_NAME="termscp-x86_64_deb"
X86_64_RPM_NAME="termscp-x86_64_rpm"

set -e # Don't fail

# Create pkgs directory
cd ..
PKGS_DIR=$(pwd)/pkgs
cd -
mkdir -p ${PKGS_DIR}/
# Build x86_64_deb
cd x86_64_debian9/
docker build $CACHE --build-arg branch=${BRANCH} --tag "$X86_64_DEB_NAME" .
cd -
mkdir -p ${PKGS_DIR}/deb/
mkdir -p ${PKGS_DIR}/x86_64-unknown-linux-gnu/
docker run --name "$X86_64_DEB_NAME" -d "$X86_64_DEB_NAME" || docker start "$X86_64_DEB_NAME"
docker exec -it "$X86_64_DEB_NAME" bash -c ". \$HOME/.cargo/env && git fetch origin && git checkout origin/$BRANCH && cargo build --release && cargo deb"
docker cp ${X86_64_DEB_NAME}:/usr/src/termscp/target/debian/termscp_${VERSION}_amd64.deb ${PKGS_DIR}/deb/
docker cp ${X86_64_DEB_NAME}:/usr/src/termscp/target/release/termscp ${PKGS_DIR}/x86_64-unknown-linux-gnu/
docker stop "$X86_64_DEB_NAME"
# Make tar.gz
cd ${PKGS_DIR}/x86_64-unknown-linux-gnu/
tar cvzf termscp-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz termscp
rm termscp
cd -
# Build x86_64_centos7
cd x86_64_centos7/
docker build $CACHE --build-arg branch=${BRANCH} --tag "$X86_64_RPM_NAME" .
cd -
mkdir -p ${PKGS_DIR}/rpm/
docker run --name "$X86_64_RPM_NAME" -d "$X86_64_RPM_NAME" || docker start "$X86_64_RPM_NAME"
docker exec -it "$X86_64_RPM_NAME" bash -c ". \$HOME/.cargo/env && git fetch origin && git checkout origin/$BRANCH; cargo rpm init; cargo rpm build"
docker cp ${X86_64_RPM_NAME}:/usr/src/termscp/target/release/rpmbuild/RPMS/x86_64/termscp-${VERSION}-1.el7.x86_64.rpm ${PKGS_DIR}/rpm/termscp-${VERSION}-1.x86_64.rpm
docker stop "$X86_64_RPM_NAME"

exit $?
