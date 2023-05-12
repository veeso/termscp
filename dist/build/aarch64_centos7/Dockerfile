FROM centos:centos7 as builder

WORKDIR /usr/src/
# Install dependencies
RUN yum -y install \
    git \
    gcc \
    pkgconfig \
    gcc \
    make \
    dbus-devel \
    libsmbclient-devel \
    bash \
    rpm-build
# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rust.sh && \
    chmod +x /tmp/rust.sh && \
    /tmp/rust.sh -y
# Clone repository
RUN git clone https://github.com/veeso/termscp.git
# Set workdir to termscp
WORKDIR /usr/src/termscp/
# Install cargo rpm
RUN source $HOME/.cargo/env && cargo install cargo-rpm

ENTRYPOINT ["tail", "-f", "/dev/null"]
