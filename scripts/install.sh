#!/usr/bin/env bash

set -e
PASSLOCK_DIR="$HOME/.passlock"

if [ -d "$PASSLOCK_DIR/.git" ]; then
  echo "Updating passlock..."
  git -C "$PASSLOCK_DIR" pull
else
  echo "Cloning passlock..."
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
  echo ""
  echo "Then restart your shell"
else
  echo "Error finding passlock directory"
fi
