#!/usr/bin/env sh

if [ ! -f docker-compose.yml ]; then
  set -e
  cd tests/
  set +e
fi

echo "Prepare volume..."
rm -rf /tmp/termscp-test-ftp
mkdir -p /tmp/termscp-test-ftp
echo "Building docker image..."
docker compose build
set -e
docker compose up -d
set +e

# Go back to src root
cd ..
# Run tests
echo "Running tests"
cargo test --features with-containers -- --test-threads 1
TEST_RESULT=$?
# Stop container
cd tests/
echo "Stopping container..."
docker compose stop

exit $TEST_RESULT
