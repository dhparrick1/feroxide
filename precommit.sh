#!/bin/bash

total_exit_code=0

export RUSTUP_NIGHTLY="yes"

CARGO="rustup run nightly cargo"

echo "Updating rust"
rustup update
total_exit_code=$((total_exit_code + $?))

echo "Updating Cargo.toml"
$CARGO update
total_exit_code=$((total_exit_code + $?))

echo "Formatting code"
$CARGO install rustfmt-nightly
$CARGO fmt
total_exit_code=$((total_exit_code + $?))

echo "Running clippy"
rustup component add clippy-preview
$CARGO clippy -- -D warnings
total_exit_code=$((total_exit_code + $?))

echo "Running tests"
$CARGO test
total_exit_code=$((total_exit_code + $?))

echo "Generating documentation"
./generate_docs.sh
total_exit_code=$((total_exit_code + $?))


if [ $total_exit_code -ne 0 ]; then
  echo "One or more steps failed. Please read the logs" >&2
else
  echo "All steps completed succesfully. You're now ready to commit!" >&2
fi

exit $total_exit_code
