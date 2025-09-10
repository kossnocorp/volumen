#!/usr/bin/env bash

set -e

echo -e "⚡️ Building Wasm package\n"

dir="../npm"

find $dir -type f \
	! -name 'package.json' \
	! -name 'turbo.json' \
	-delete

wasm-pack build --target nodejs --out-name volumen --out-dir $dir

version=$(jaq -r ".version" "$package_json")

echo -e "$(cat "$dir/package.json" | jaq '
  .dependencies //= {} |
  .dependencies["@volumen/types"] = "workspace:^" |
  .name = "volumen" |
  .version = "'"$version"'" |
')" >"$dir/package.json"

cat >"$dir/.gitignore" <<'EOF'
*.js
*.d.ts
*.wasm
EOF
