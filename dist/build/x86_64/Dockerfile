FROM rust:1.48.0 AS builder

WORKDIR /usr/src/
# Add toolchains
RUN rustup target add x86_64-unknown-linux-gnu
# Install dependencies
RUN apt update && apt install -y rpm
# Clone repository
RUN git clone https://github.com/ChristianVisintin/termscp.git
# Set workdir to termscp
WORKDIR /usr/src/termscp/
# Install cargo RPM/Deb
RUN cargo install cargo-deb cargo-rpm
# Build for x86_64
RUN cargo build --release --target x86_64-unknown-linux-gnu
# Build pkgs
RUN cargo deb && cargo rpm init && cargo rpm build

CMD ["sh"]
