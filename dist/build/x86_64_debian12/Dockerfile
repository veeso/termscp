FROM debian:bookworm

WORKDIR /usr/src/
# Install dependencies
RUN apt update && apt install -y \
  git \
  gcc \
  pkg-config \
  libdbus-1-dev \
  build-essential \
  libsmbclient-dev \
  libgit2-dev \
  build-essential \
  pkg-config \
  libbsd-dev \
  libcap-dev \
  libcups2-dev \
  libgnutls28-dev \
  libgnutls30 \
  libicu-dev \
  libjansson-dev \
  libkeyutils-dev \
  libldap2-dev \
  zlib1g-dev \
  libpam0g-dev \
  libacl1-dev \
  libarchive-dev \
  libssl-dev \
  flex \
  bison \
  libntirpc-dev \
  libglib2.0-dev \
  libdbus-1-dev \
  libsasl2-dev \
  libunistring-dev \
  bash \
  curl \
  cpanminus && \
  cpanm Parse::Yapp::Driver;

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rust.sh && \
  chmod +x /tmp/rust.sh && \
  /tmp/rust.sh -y && \
  . $HOME/.cargo/env && \
  cargo version
# Clone repository
RUN git clone https://github.com/veeso/termscp.git
# Set workdir to termscp
WORKDIR /usr/src/termscp/
# Install cargo deb
RUN . $HOME/.cargo/env && cargo install cargo-deb

ENTRYPOINT ["tail", "-f", "/dev/null"]
