#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: deploy.sh <version>"
    exit 1
fi

VERSION=$1

# Build x86_64
cd x86_64/
docker build --tag termscp-${VERSION}-x86_64 .
# Get pkgs
cd -
# Create container
CONTAINER_NAME=$(docker create termscp-${VERSION}-x86_64 termscp-${VERSION}-x86_64)
docker cp ${CONTAINER_NAME}:/usr/src/TermSCP/target/debian/termscp_${VERSION}_amd64.deb .
docker cp ${CONTAINER_NAME}:/usr/src/TermSCP/target/release/rpmbuild/RPMS/x86_64/termscp-${VERSION}-1.x86_64.rpm .

exit $?
