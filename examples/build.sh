#!/usr/bin/env bash
set -e

package="$1"

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


app_root=$(get_package_path "$package")
did_file="$app_root/$package.did"

# This script generates the did file, build the project (passed as $1) and then run the ic-wasm to shrink and attach metadata.
cargo build --manifest-path="$app_root/Cargo.toml" \
    --target wasm32-unknown-unknown \
    --release \
    --package "$package"

candid-extractor "./target/wasm32-unknown-unknown/release/$package.wasm" 2>/dev/null > $did_file

ic-wasm "./target/wasm32-unknown-unknown/release/$package.wasm" \
    -o "./target/wasm32-unknown-unknown/release/$package.wasm" \
    metadata candid:service -v public -f $did_file

ic-wasm "./target/wasm32-unknown-unknown/release/$package.wasm" \
    -o "./target/wasm32-unknown-unknown/release/$package-opt.wasm" \
    shrink

