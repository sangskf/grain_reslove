#!/bin/bash

# Function to increment the version number
increment_version() {
    local version=$1
    if [[ $version =~ ([0-9]+)\.([0-9]+)\.([0-9]+) ]]; then
        local major=${BASH_REMATCH[1]}
        local minor=${BASH_REMATCH[2]}
        local patch=${BASH_REMATCH[3]}
        local new_patch=$((patch + 1))
        echo "$major.$minor.$new_patch"
    else
        echo "$version"
    fi
}

# Get current version from package.json
current_version=$(grep -o '"version": "[^"]*"' package.json | cut -d'"' -f4)
new_version=$(increment_version "$current_version")

echo "Updating version from $current_version to $new_version"

# Update version in package.json
sed -i.bak "s/\"version\": \"$current_version\"/\"version\": \"$new_version\"/" package.json

# Update version in src-tauri/Cargo.toml
sed -i.bak "s/^version = \"$current_version\"/version = \"$new_version\"/" src-tauri/Cargo.toml

# Update version in src-tauri/tauri.conf.json
sed -i.bak "s/\"version\": \"$current_version\"/\"version\": \"$new_version\"/" src-tauri/tauri.conf.json

# Update endpoints in tauri.conf.json
sed -i.bak "s/v$current_version\/latest.json/v$new_version\/latest.json/g" src-tauri/tauri.conf.json

echo "Version updated to $new_version in all files"

# Clean up backup files
rm -f package.json.bak src-tauri/Cargo.toml.bak src-tauri/tauri.conf.json.bak
