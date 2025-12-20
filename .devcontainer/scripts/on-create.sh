#!/usr/bin/env bash

# This script is run when the container is created.

set -e

# Install clang needed for building Tree-sitter to Wasm.
sudo apt-get update && sudo apt-get install -y clang