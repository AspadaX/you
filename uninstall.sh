#!/bin/bash

set -euo pipefail

# Installation locations
BINARY_PATH="/usr/local/bin/you"
CONFIG_DIRS=(
    "/etc/you"
)

# Remove main binary
if [[ -f "$BINARY_PATH" ]]; then
    echo "Removing you binary..."
    sudo rm -f "$BINARY_PATH"
fi

# Remove configuration directories
for dir in "${CONFIG_DIRS[@]}"; do
    if [[ -d "$dir" ]]; then
        echo "Removing configuration directory: $dir"
        rm -rf "$dir"
    fi
done

# Verify removal
if ! command -v you &> /dev/null; then
    echo "you was successfully uninstalled"
else
    echo "Warning: Some you components might still remain"
    exit 1
fi