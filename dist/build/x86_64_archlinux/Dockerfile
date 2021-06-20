FROM archlinux:base-20210120.0.13969 as builder

WORKDIR /usr/src/
# Install dependencies
RUN pacman -Syu --noconfirm \
    git \
    gcc \
    openssl \
    pkg-config \
    dbus \
    sudo
# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rust.sh && \
    chmod +x /tmp/rust.sh && \
    /tmp/rust.sh -y
# Create build user
RUN useradd build -m && \
    passwd -d build && \
    mkdir -p termscp && \
    chown -R build.build termscp/
# Clone repository
RUN git clone https://github.com/veeso/termscp.git
# Set workdir to termscp
WORKDIR /usr/src/termscp/
# Install cargo arxch
RUN source $HOME/.cargo/env && cargo install cargo-aur
# Build for x86_64
RUN source $HOME/.cargo/env && cargo build --release
# Build pkgs
RUN source $HOME/.cargo/env && cargo aur
# Create SRCINFO
RUN chown -R build.build ../termscp/ && sudo -u build bash -c 'makepkg --printsrcinfo > .SRCINFO'

CMD ["sh"]
