#!/usr/bin/env bash

. "$(git rev-parse --show-toplevel)/scripts/set-cargo-env.sh"

cargo fetch

docker run --user "$(id -u)":"$(id -g)" -t --rm -v "$(pwd):/workdir" --workdir "/workdir" -e "CARGO_HOME=/workdir/.cargo" rustlang/rust:nightly "cargo" "build" "--release"

docker build -f docker/binary.Dockerfile -t rocket-add-binary .
