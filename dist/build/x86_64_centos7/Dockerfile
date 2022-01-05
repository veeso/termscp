FROM centos:centos7 as builder

ARG branch
ENV branch=$branch
WORKDIR /usr/src/
# Install dependencies
RUN yum -y install \
    git \
    gcc \
    openssl \
    pkgconfig \
    dbus-devel \
    openssl-devel \
    bash
# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rust.sh && \
    chmod +x /tmp/rust.sh && \
    /tmp/rust.sh -y
# Clone repository
RUN git clone --branch $branch https://github.com/veeso/termscp.git
# Set workdir to termscp
WORKDIR /usr/src/termscp/
# Install cargo arxch
RUN source $HOME/.cargo/env && cargo install cargo-rpm cargo-strip
# Build for x86_64
RUN source $HOME/.cargo/env && cargo build --release && cargo strip
# Build pkgs
RUN source $HOME/.cargo/env && yum -y install rpm-build && cargo rpm init && cargo rpm build
CMD ["sh"]
