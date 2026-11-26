#!/usr/bin/env bash
set -e
REPO="https://github.com/justzac2026/ZPlayer.git"
echo "YEHEY! you are installing ZPlayer."
if ! command -v cargo >/dev/null; then
    echo "Rust and Cargo are not installed. Please install them from https://rustup.rs"
    exit 1
fi
cargo install --git "$REPO"
echo "Successfully installed ZPlayer! Run this command to start it: zplayer"