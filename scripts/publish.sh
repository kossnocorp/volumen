#!/usr/bin/env bash

set -e

echo -e "⚡️ Publishing packages\n"

echo -e "🌀 Packaging...\n"
pnpm turbo run package
echo

echo -e "🌀 Publishing crates...\n"
cargo release publish --no-confirm --no-verify
echo

echo -e "🌀 Publishing npm packages...\n"
pnpm publish --recursive --no-git-checks --dry-run
echo
