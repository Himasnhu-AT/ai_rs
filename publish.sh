#!/bin/bash

set -e

echo "==> Checking for uncommitted changes..."
if [[ -n $(git status --porcelain) ]]; then
  echo "Warning: You have uncommitted changes."
  git status
  read -p "Continue anyway? (y/N): " yn
  case $yn in
      [Yy]*) ;;
      *) echo "Aborting."; exit 1;;
  esac
fi

echo "==> Running tests..."
cargo test

echo "==> Building package..."
cargo build --release

echo "==> Checking package..."
cargo check

echo "==> Please ensure you have logged in to crates.io with 'cargo login <token>'"
read -p "Ready to publish to crates.io? (y/N): " yn
case $yn in
    [Yy]*) ;;
    *) echo "Aborting."; exit 1;;
esac

echo "==> Publishing to crates.io..."
cargo publish

echo "==> Done! Package published."