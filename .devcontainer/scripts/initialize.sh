#!/usr/bin/env bash

set -e

# This script is run on THE HOST when the source code is located.
echo -e "⚡️ Bootstrapping host directories and files...\n"

devcontainer_id=$1
if [ -z "$devcontainer_id" ]; then
	echo '🔴 No devcontainer ID provided. Usage: .devcontainer/scripts/initialize.sh ${devcontainerId} ${localWorkspaceFolderBasename}' >&2
	exit 1
fi

wrkspc_dir_name=$2
if [ -z "$wrkspc_dir_name" ]; then
	echo '🔴 No local workspace dir name provided. Usage: .devcontainer/scripts/initialize.sh ${devcontainerId} ${localWorkspaceFolderBasename}' >&2
	exit 1
fi

echo -e "\n🌀 Ensuring state directories"

state_dir="$HOME/.local/state/mothership/containers/$devcontainer_id"
mkdir -p "$state_dir"

dirs=(
	".cache"
	".local/share"
	".local/state"
	".rustup"
	".codex/sessions"
	"wrkspc/$wrkspc_dir_name/node_modules"
)

for rel_dir in "${dirs[@]}"; do
	dir="$state_dir/$rel_dir"
	mkdir -p "$dir"
	echo "🔹 $rel_dir"
done

echo

echo -e "🌀 Ensuring host directories"

dirs=(
	".cargo"
	".codex"
)

for rel_dir in "${dirs[@]}"; do
	dir="$HOME/$rel_dir"
	mkdir -p "$dir"
	echo "🔹 $rel_dir"
done

echo

echo -e "🌀 Ensuring host files"

ensure_file() {
	file="$HOME/$1"
	content="$2"
	echo "🔹 $file"
	mkdir -p "$(dirname "$file")"
	[ -f "$file" ] || echo "$content" >"$file"
}

ensure_file ".cargo/credentials.toml"
ensure_file ".codex/auth.json" "{}"
ensure_file ".npmrc"
ensure_file ".config/mothership/.env"

echo

echo -e "🟢 Host bootstrapped!"
