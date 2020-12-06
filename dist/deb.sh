#!/bin/bash

echo "Installing cargo-deb..."
cargo install cargo-deb
if [ ! -f "Cargo.toml" ]; then
    echo "Yout must be in the project root directory"
    exit 1
fi
echo "Running cargo-deb"
cargo deb
exit $?
