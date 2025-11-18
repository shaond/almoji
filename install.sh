#!/bin/bash
# Simple installation script for Almoji

set -e

echo "Installing Almoji..."
echo ""

# Detect OS
OS="$(uname -s)"

# Check if we should install system-wide
if [ "$1" = "--system" ] || [ "$1" = "-s" ]; then
    echo "Installing to system directory (/usr/local/bin)..."

    # Install CLI
    cargo install --path . --bin almoji --root /usr/local
    echo "✓ Almoji CLI installed to /usr/local/bin/almoji"

    # Install menubar app on macOS
    if [ "$OS" = "Darwin" ]; then
        cargo install --path . --bin almoji-menubar --root /usr/local
        echo "✓ Almoji Menubar installed to /usr/local/bin/almoji-menubar"
    fi
else
    echo "Installing to user directory (~/.cargo/bin)..."

    # Install CLI
    cargo install --path . --bin almoji
    echo "✓ Almoji CLI installed to ~/.cargo/bin/almoji"

    # Install menubar app on macOS
    if [ "$OS" = "Darwin" ]; then
        cargo install --path . --bin almoji-menubar
        echo "✓ Almoji Menubar installed to ~/.cargo/bin/almoji-menubar"
    fi

    echo ""
    echo "Make sure ~/.cargo/bin is in your PATH"
fi

echo ""
echo "CLI usage: almoji fire"

# Show menubar app instructions on macOS
if [ "$OS" = "Darwin" ]; then
    echo ""
    echo "============================="
    echo "macOS Menubar App Available!"
    echo "============================="
    echo ""
    echo "To run the menubar app:"
    if [ "$1" = "--system" ] || [ "$1" = "-s" ]; then
        echo "  /usr/local/bin/almoji-menubar"
    else
        echo "  almoji-menubar"
    fi
    echo ""
    echo "The menubar app will:"
    echo "  • Add a 😊 icon to your menubar"
    echo "  • Let you search emojis with Cmd+Shift+E"
    echo "  • Automatically copy selected emojis"
    echo ""
    echo "To run at login, add almoji-menubar to your Login Items in System Settings."
fi

echo ""
