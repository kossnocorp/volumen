#!/usr/bin/env bash

set -e

version=$1
if [ -z "$version" ]; then
	echo '🔴 No version provided. Usage: `version.sh <version>`' >&2
	exit 1
fi

echo -e "⚡️ Bumping version to v$version\n"

echo -e "🌀 Setting crates versions...\n"
cargo release version $version --execute --no-confirm
echo

echo -e "🌀 Setting npm packages versions...\n"
for pkg in ./pkgs/*; do
	package_json="$pkg/package.json"

	# Skip ignored, ensure that's not a dir, has package.json and it's not private.
	[ "$pkg" = "./pkgs/types" ] && continue
	[ -d "$dir" ] && continue
	[ -f "$package_json" ] || continue
	jaq -e '.private == true' "$package_json" >/dev/null && continue

	pkg_name=$(jaq -r ".name" "$package_json")
	echo "🔹 Setting $pkg_name..."
	pnpm version $version --no-git-tag-version --allow-same-version >/dev/null
done
echo

echo -e "🌀 Setting types versions...\n"
sed -i "s|^version = \".*\"|version = \"$version\"|" pkgs/types/genotype.toml

echo -e "🟢 Version set to v$version!"
