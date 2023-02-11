FROM centos:centos7 as builder

ARG branch
ENV branch=$branch
WORKDIR /usr/src/
# Install dependencies
RUN yum -y install \
    git \
    gcc \
    pkgconfig \
    gcc \
    make \
    dbus-devel \
    bash \
    rpm-build
# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rust.sh && \
    chmod +x /tmp/rust.sh && \
    /tmp/rust.sh -y
# Clone repository
RUN git clone --branch $branch https://github.com/veeso/termscp.git
# Set workdir to termscp
WORKDIR /usr/src/termscp/
# Build for x86_64
RUN source $HOME/.cargo/env && cargo build --release
# Install cargo rpm
RUN source $HOME/.cargo/env && cargo install cargo-rpm
# Build pkgs
RUN source $HOME/.cargo/env && cargo rpm init && cargo rpm build
CMD ["sh"]
