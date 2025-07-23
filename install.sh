#!/bin/sh
set -e

tmp_dir=$(mktemp -d)
echo "Cloning repo to $tmp_dir..."
git clone https://github.com/your-username/rps.git "$tmp_dir"

cd "$tmp_dir"

echo "Building the binary..."
cargo build --release

echo "Installing to ~/.cargo/bin..."
mkdir -p ~/.cargo/bin
cp target/release/rps ~/.cargo/bin/

echo "Cleaning up..."
rm -rf "$tmp_dir"

echo "Done! You can now play the game by typing: rps"
