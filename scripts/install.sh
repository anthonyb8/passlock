#!/usr/bin/env bash

set -e

PASSLOCK_DIR="$HOME/.passlock"
REPO_URL="https://github.com/anthonyb8/passlock.git"

mkdir -p "$PASSLOCK_DIR"

if [ -d "$PASSLOCK_DIR/.git" ]; then
  echo "Updating passlock..."
  git -C "$PASSLOCK_DIR" pull
else
  echo "Cloning passlock..."
  rm -rf "$PASSLOCK_DIR" # clean out any old directories
  git clone "$REPO_URL" "$PASSLOCK_DIR"
fi

if cd "$PASSLOCK_DIR"; then
  echo "Building passlock..."
  cargo build --release

  echo "Installing passlock..."
  mkdir -p ~/.passlock/bin
  cp target/release/passlock ~/.passlock/bin/

  echo ""
  echo "To use 'passlock' from anywhere, add this to your shell config:"
  echo ""
  echo "    export PATH=\"\$HOME/.passlock/bin:\$PATH\""
  echo "or run: "
  echo "    echo 'export PATH=\"\$HOME/.passlock/bin:\$PATH\"' >> ~/.zshrc "
  echo ""
  echo "Then restart your shell"
else
  echo "Error finding passlock directory"
  exit 1
fi
