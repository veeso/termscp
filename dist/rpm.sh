#!/bin/bash

which rpmbuild > /dev/null
if [ $? -ne 0 ]; then
    echo "You must install rpmbuild on your machine"
fi
echo "Installing cargo-rpm..."
cargo install cargo-rpm
if [ ! -f "Cargo.toml" ]; then
    echo "Yout must be in the project root directory"
    exit 1
fi
echo "Running cargo-rpm"
cargo rpm init
cargo rpm build
exit $?
