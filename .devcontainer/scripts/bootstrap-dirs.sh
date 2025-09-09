#!/usr/bin/env bash

# This script bootstraps state directories in the host environment, to enable
# proper volume mounting. It must be run on the host machine before starting
# the devcontainer.

set -e

echo "⚡️ Bootstrapping state directories...\n"

# # Check if git is available and show an error if not
# if ! command -v git >/dev/null 2>&1; then
#     printf "❌ git is required but not installed or not in PATH.\n" >&2
#     exit 1
# fi

# Ensure we're inside a git repository before adding files
if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
	echo "🔴 Not a git repository. Initialize with:\n\n    git init\n" >&2
	exit 1
fi

state_dir=".devcontainer/state"

dirs=(
	".cache"
	".local/share"
	".local/state"
	".codex/sessions"
)

gitkeeps=()
for rel_dir in "${dirs[@]}"; do
	dir="$state_dir/$rel_dir"
	gitkeep="$dir/.gitkeep"
	mkdir -p "$dir"
	touch "$gitkeep"
	git add -f "$gitkeep"
	echo "🟢 $gitkeep"
done
