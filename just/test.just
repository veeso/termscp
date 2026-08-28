# Run all tests
[group('test')]
test_all: test

# Run the Rust test suite
[group('test')]
test args="":
  cargo test --workspace {{ args }}

# Generate an LCOV code coverage report for the Rust workspace (requires cargo-llvm-cov)
[group('test')]
coverage output="lcov.info":
  cargo llvm-cov --workspace --lcov --output-path {{ output }}
