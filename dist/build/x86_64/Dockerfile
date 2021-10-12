FROM rust:1.55.0 AS builder

WORKDIR /usr/src/
# Add toolchains
RUN rustup target add x86_64-unknown-linux-gnu
# Install dependencies
RUN apt update && apt install -y \
  git \
  gcc \
  pkg-config \
  libssl-dev \
  libssh2-1-dev \
  libdbus-1-dev \
  curl
# Clone repository
RUN git clone https://github.com/veeso/termscp.git
# Set workdir to termscp
WORKDIR /usr/src/termscp/
# Install cargo RPM/Deb
RUN cargo install cargo-strip
# Build for x86_64
RUN cargo build --release --target x86_64-unknown-linux-gnu && cargo strip

CMD ["sh"]
