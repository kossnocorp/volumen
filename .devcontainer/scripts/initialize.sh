#!/usr/bin/env bash

set -e

# This script is run on THE HOST when the source code is located.
echo "⚡️ Bootstrapping state directories...\n"

devcontainer_id=$1
if [ -z "$devcontainer_id" ]; then
	echo "🔴 No devcontainer ID provided. Usage: ./initialize.sh <devcontainer-id>" >&2
	exit 1
fi

state_dir="$HOME/.local/state/mothership/containers/$devcontainer_id"
mkdir -p "$state_dir"

dirs=(
	".cache"
	".local/share"
	".local/state"
	".codex/sessions"
)

gitkeeps=()
for rel_dir in "${dirs[@]}"; do
	dir="$state_dir/$rel_dir"
	mkdir -p "$dir"
	echo "🟢 $rel_dir"
done
