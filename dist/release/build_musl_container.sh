#!/usr/bin/env sh
# Builds a static musl termscp binary. Runs INSIDE the Alpine container
# started by dist/release/build_musl.sh; /work is the mounted workspace.
#
# Required environment: TARGET, HOST_UID, HOST_GID.
set -eux

NETTLE_VERSION="3.10.1"
GNUTLS_VERSION="3.8.13"
PAVAO_SRC_VERSION="4.24.6"

cleanup() {
  chown -R "$HOST_UID:$HOST_GID" /work
}
trap cleanup EXIT

apk add --no-cache \
  bison \
  binutils \
  build-base \
  file \
  flex \
  git \
  gnutls-dev \
  libgit2-dev \
  libgit2-static \
  libunistring-dev \
  libunistring-static \
  linux-headers \
  openssl-dev \
  openssl-libs-static \
  perl \
  perl-parse-yapp \
  pkgconf \
  python3 \
  wget \
  xz \
  zlib-dev \
  zlib-static

rustup target add "$TARGET"
cargo fetch --locked

NATIVE_CFLAGS="-O2 -fPIC"
if [ "$TARGET" = "aarch64-unknown-linux-musl" ]; then
  NATIVE_CFLAGS="$NATIVE_CFLAGS -mno-outline-atomics"
fi
export CFLAGS="$NATIVE_CFLAGS"

# -- static nettle (GnuTLS crypto backend); mini-gmp avoids a GMP dependency
mkdir -p /tmp/native
wget -q "https://ftp.gnu.org/gnu/nettle/nettle-$NETTLE_VERSION.tar.gz" \
  -O /tmp/native/nettle.tar.gz
tar -xzf /tmp/native/nettle.tar.gz -C /tmp/native
cd "/tmp/native/nettle-$NETTLE_VERSION"
./configure \
  --prefix=/tmp/native/nettle \
  --disable-shared \
  --enable-static \
  --disable-documentation \
  --enable-mini-gmp
make -j"$(getconf _NPROCESSORS_ONLN)"
make install

# -- static GnuTLS; every optional backend is disabled so nothing links
#    against a shared library
wget -q "https://www.gnupg.org/ftp/gcrypt/gnutls/v3.8/gnutls-$GNUTLS_VERSION.tar.xz" \
  -O /tmp/native/gnutls.tar.xz
tar -xf /tmp/native/gnutls.tar.xz -C /tmp/native
cd "/tmp/native/gnutls-$GNUTLS_VERSION"
PKG_CONFIG_PATH=/tmp/native/nettle/lib/pkgconfig \
  ./configure \
    --prefix=/tmp/native/gnutls \
    --disable-shared \
    --enable-static \
    --disable-doc \
    --disable-tests \
    --disable-nls \
    --disable-hardware-acceleration \
    --with-nettle-mini \
    --with-included-libtasn1 \
    --with-included-unistring \
    --without-idn \
    --without-p11-kit \
    --without-brotli \
    --without-zstd \
    --without-zlib
make -j"$(getconf _NPROCESSORS_ONLN)"
make install

# -- flatten gnutls.pc: pkg-config must hand the linker the static archives
#    directly, with no Requires.private chain to resolve
mkdir -p /tmp/native/pkgconfig
sed \
  -e "s#^Libs:.*#Libs: -L/tmp/native/gnutls/lib -lgnutls -latomic -L/tmp/native/nettle/lib -lhogweed -lnettle#" \
  -e "/^Requires.private:/d" \
  -e "s#^Cflags:.*#Cflags: -I/tmp/native/gnutls/include -I/tmp/native/nettle/include#" \
  /tmp/native/gnutls/lib/pkgconfig/gnutls.pc \
  > /tmp/native/pkgconfig/gnutls.pc

# -- pavao-src: Samba's replacement library omits two sources that musl needs
cd /work
PAVAO_SRC=$(find "${CARGO_HOME:-/usr/local/cargo}/registry/src" \
  -type d -name "pavao-src-$PAVAO_SRC_VERSION" -print -quit)
test -n "$PAVAO_SRC"
cp -R "$PAVAO_SRC" /tmp/pavao-src
perl -0pi -e "s#(    \\\"lib/replace/replace\\.c\\\",\\n)#\$1    \\\"lib/replace/closefrom.c\\\",\\n    \\\"lib/replace/strptime.c\\\",\\n#" \
  /tmp/pavao-src/src/lib.rs

cat >> Cargo.toml <<EOF

[patch.crates-io]
pavao-src = { path = "/tmp/pavao-src" }
EOF
cargo update -p "pavao-src@$PAVAO_SRC_VERSION"

export PKG_CONFIG_ALL_STATIC=1
export PKG_CONFIG_PATH=/tmp/native/pkgconfig:/tmp/native/nettle/lib/pkgconfig:/usr/lib/pkgconfig
export RUSTFLAGS="-C target-feature=+crt-static -C link-arg=-static"
cargo build --locked --release --target "$TARGET" --features smb-vendored

# -- prove the binary is static: no interpreter, no shared libraries
file "target/$TARGET/release/termscp"
if readelf -l "target/$TARGET/release/termscp" > /tmp/program-headers.txt; then
  cat /tmp/program-headers.txt
else
  status=$?
  cat /tmp/program-headers.txt
  exit "$status"
fi
if readelf -d "target/$TARGET/release/termscp" > /tmp/dynamic-section.txt; then
  cat /tmp/dynamic-section.txt
else
  status=$?
  cat /tmp/dynamic-section.txt
  exit "$status"
fi
if grep -q INTERP /tmp/program-headers.txt; then
  exit 1
fi
if grep -q NEEDED /tmp/dynamic-section.txt; then
  exit 1
fi
