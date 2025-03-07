#!/bin/sh

set -e

(
  cd "$(dirname "$0")"
  cargo build \
      --quiet \
      --release \
      --target-dir=/tmp/rlox-interpreter-target \
      --manifest-path Cargo.toml
)

exec /tmp/rlox-interpreter-target/release/lox-interpreter-rust "$@"
