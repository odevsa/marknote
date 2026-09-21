#!/usr/bin/env bash

set -e

NEW_VERSION=$1

if [ -z "$NEW_VERSION" ]; then
  echo "Usage: ./scripts/set-version.sh <new_version>"
  echo "Example: ./scripts/set-version.sh 0.1.0"
  exit 1
fi

# Strip leading 'v' if provided
NEW_VERSION=${NEW_VERSION#v}

echo "Updating project version to: $NEW_VERSION"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

# 1. Update root package.json
sed -i -E "s/\"version\": \"[0-9]+\.[0-9]+\.[0-9]+\"/\"version\": \"$NEW_VERSION\"/g" "$ROOT_DIR/package.json"

# 2. Update frontend/package.json
sed -i -E "s/\"version\": \"[0-9]+\.[0-9]+\.[0-9]+\"/\"version\": \"$NEW_VERSION\"/g" "$ROOT_DIR/frontend/package.json"

# 3. Update frontend/src/lib/version.ts
echo "export const APP_VERSION = '$NEW_VERSION';" > "$ROOT_DIR/frontend/src/lib/version.ts"

# 4. Update backend/Cargo.toml
sed -i -E "0,/version = \"[0-9]+\.[0-9]+\.[0-9]+\"/s//version = \"$NEW_VERSION\"/" "$ROOT_DIR/backend/Cargo.toml"

# 5. Run cargo check to sync Cargo.lock
(cd "$ROOT_DIR/backend" && cargo check --quiet)

echo "Version successfully updated to $NEW_VERSION across all project files!"
