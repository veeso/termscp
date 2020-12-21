FROM archlinux/archlinux:latest as builder

WORKDIR /usr/src/
# Install dependencies
RUN pacman -Syu --noconfirm \
    git \
    gcc \
    openssl \
    pkg-config
# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rust.sh && \
    chmod +x /tmp/rust.sh && \
    /tmp/rust.sh -y
# Clone repository
RUN git clone https://github.com/ChristianVisintin/termscp.git
# Set workdir to termscp
WORKDIR /usr/src/termscp/
# Install cargo arxch
RUN source $HOME/.cargo/env && cargo install cargo-aur
# Build for x86_64
RUN source $HOME/.cargo/env && cargo build --release
# Build pkgs
RUN source $HOME/.cargo/env && cargo aur

CMD ["sh"]
