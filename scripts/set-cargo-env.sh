#!/usr/bin/env bash

export CARGO_HOME="$(git rev-parse --show-toplevel)/.cargo"
echo "Creating local cargo config with $CARGO_HOME"
mkdir -p $CARGO_HOME

# Use host `git` command to clone dependencies
cat << EOF > "$CARGO_HOME/config"
[net]
git-fetch-with-cli = true
EOF

rustup override set nightly
rustup show
