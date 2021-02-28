FROM archlinux/archlinux:base-20210110.0.13332 as builder

WORKDIR /usr/src/
# Install dependencies
RUN pacman -Syu --noconfirm \
    git \
    gcc \
    openssl \
    pkg-config \
    sudo
# Create build user
RUN useradd build -m && \
    passwd -d build && \
    mkdir -p termscp && \
    chown -R build.build termscp/
# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust.sh && \
    chmod +x ./rust.sh && \
    ./rust.sh -y
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
