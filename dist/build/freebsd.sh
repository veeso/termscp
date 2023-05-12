#!/bin/sh

if [ -z "$1" ]; then
    echo "Usage: freebsd.sh <version>"
    exit 1
fi

VERSION=$1

set -e # Don't fail

# Go to root dir
cd ../../
# Check if in correct directory
if [ ! -f Cargo.toml ]; then
    echo "Please start freebsd.sh from dist/build/ directory"
    exit 1
fi

# Build release
cargo build --release && cargo strip
# Make pkg
cd target/release/
PKG="termscp-v${VERSION}-x86_64-unknown-freebsd.tar.gz"
tar czf $PKG termscp
sha256sum $PKG
# Calc sha256 of exec and copy to path
HASH=`sha256sum termscp | cut -d ' ' -f1`
sudo cp termscp /usr/local/bin/termscp
mkdir -p ../../dist/pkgs/freebsd/
mv $PKG ../../dist/pkgs/freebsd/$PKG
cd ../../dist/pkgs/freebsd/
rm manifest
echo -e "name: \"termscp\"" > manifest
echo -e "version: $VERSION" >> manifest
echo -e "origin: veeso/termscp" >> manifest
echo -e "comment: \"A feature rich terminal UI file transfer and explorer with support for SCP/SFTP/FTP/S3\"" >> manifest
echo -e "desc: <<EOD\n\
    A feature rich terminal UI file transfer and explorer with support for SCP/SFTP/FTP/S3\n\
EOD\n\
arch: \"amd64\"\n\
www: \"https://termscp.veeso.dev/termscp/\"\n\
maintainer: \"christian.visintin1997@gmail.com\"\n\
prefix: \"/usr/local/bin\"\n\
deps: {\n\
  libssh: {origin: security/libssh, version: 0.9.5}\n\
}\n\
files: {\n\
  /usr/local/bin/termscp: \"$HASH\"\n\
}\n\
" >> manifest

exit $?
