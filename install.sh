#!/bin/bash
# Simple installation script for Almoji

set -e

echo "Installing Almoji..."
echo ""

# Check if we should install system-wide
if [ "$1" = "--system" ] || [ "$1" = "-s" ]; then
    echo "Installing to system directory (/usr/local/bin)..."
    cargo install --path . --root /usr/local
    echo ""
    echo "✓ Almoji installed to /usr/local/bin/almoji"
else
    echo "Installing to user directory (~/.cargo/bin)..."
    cargo install --path .
    echo ""
    echo "✓ Almoji installed to ~/.cargo/bin/almoji"
    echo ""
    echo "Make sure ~/.cargo/bin is in your PATH"
fi

echo ""
echo "Try it: almoji fire"
