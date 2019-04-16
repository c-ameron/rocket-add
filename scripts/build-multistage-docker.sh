#!/usr/bin/env bash

. "$(git rev-parse --show-toplevel)/scripts/set-cargo-env.sh"

cargo fetch

docker build -f docker/Dockerfile -t rocket-add .
