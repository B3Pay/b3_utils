#!/usr/bin/env bash

function get_package_path() {
    local package_name=$1
    local member_lines=$(grep -E '^    "[^"]+"' Cargo.toml)
    echo "$member_lines" | while read -r line; do
        # Use awk to extract the path, removing the quotes
        local member_path=$(echo "$line" | awk -F '"' '{print $2}')
        # Check if the member_path contains the package_name
        if [[ "$member_path" == *"$package_name"* ]]; then
            echo "$member_path"
            break
        fi
    done
}

# Usage
package="$1"
app_root=$(get_package_path "$package")
did_file="$app_root/$package.did"

echo "package: $package"
echo "app_root: $app_root"
echo "did_file: $did_file"