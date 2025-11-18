#!/bin/bash
# Installation script for Almoji

set -e

echo "Installing Almoji..."
echo ""

# Detect OS
OS="$(uname -s)"

# Determine installation directory
if [ "$1" = "--system" ] || [ "$1" = "-s" ]; then
    INSTALL_MODE="system"
    INSTALL_DIR="/usr/local/bin"
    echo "Installation mode: System-wide ($INSTALL_DIR)"
else
    INSTALL_MODE="user"
    INSTALL_DIR="~/.cargo/bin"
    echo "Installation mode: User directory ($INSTALL_DIR)"
fi

echo ""
echo "========================================="
echo "Installing CLI Application"
echo "========================================="

# Install CLI - this should always succeed
if [ "$INSTALL_MODE" = "system" ]; then
    if cargo install --path . --bin almoji --root /usr/local; then
        echo "✅ SUCCESS: Almoji CLI installed to /usr/local/bin/almoji"
        CLI_SUCCESS=true
    else
        echo "❌ FAILED: Could not install Almoji CLI"
        CLI_SUCCESS=false
    fi
else
    if cargo install --path . --bin almoji; then
        echo "✅ SUCCESS: Almoji CLI installed to ~/.cargo/bin/almoji"
        CLI_SUCCESS=true
    else
        echo "❌ FAILED: Could not install Almoji CLI"
        CLI_SUCCESS=false
    fi
fi

echo ""

# Only attempt menubar installation on macOS
if [ "$OS" = "Darwin" ]; then
    echo "========================================="
    echo "Installing Menubar Application (macOS)"
    echo "========================================="

    # Install menubar app - allow this to fail without stopping the script
    set +e  # Temporarily disable exit on error

    if [ "$INSTALL_MODE" = "system" ]; then
        if cargo install --path . --bin almoji-menubar --root /usr/local 2>&1; then
            echo "✅ SUCCESS: Almoji Menubar installed to /usr/local/bin/almoji-menubar"
            MENUBAR_SUCCESS=true
        else
            echo "❌ FAILED: Could not install Almoji Menubar app"
            echo "   (This is optional - CLI installation still succeeded)"
            MENUBAR_SUCCESS=false
        fi
    else
        if cargo install --path . --bin almoji-menubar 2>&1; then
            echo "✅ SUCCESS: Almoji Menubar installed to ~/.cargo/bin/almoji-menubar"
            MENUBAR_SUCCESS=true
        else
            echo "❌ FAILED: Could not install Almoji Menubar app"
            echo "   (This is optional - CLI installation still succeeded)"
            MENUBAR_SUCCESS=false
        fi
    fi

    set -e  # Re-enable exit on error
    echo ""
fi

echo "========================================="
echo "Installation Summary"
echo "========================================="

if [ "$CLI_SUCCESS" = true ]; then
    echo "✅ CLI: Installed successfully"
else
    echo "❌ CLI: Installation failed"
fi

if [ "$OS" = "Darwin" ]; then
    if [ "$MENUBAR_SUCCESS" = true ]; then
        echo "✅ Menubar App: Installed successfully"
    else
        echo "❌ Menubar App: Installation failed (optional)"
    fi
fi

echo ""

if [ "$INSTALL_MODE" = "user" ]; then
    echo "Make sure ~/.cargo/bin is in your PATH"
    echo ""
fi

# Show usage instructions
if [ "$CLI_SUCCESS" = true ]; then
    echo "========================================="
    echo "CLI Usage"
    echo "========================================="
    echo "  almoji fire"
    echo "  almoji heart"
    echo ""
fi

# Show menubar app instructions on macOS if installed successfully
if [ "$OS" = "Darwin" ] && [ "$MENUBAR_SUCCESS" = true ]; then
    echo "========================================="
    echo "Menubar App Usage (macOS)"
    echo "========================================="
    echo ""
    echo "To run the menubar app:"
    if [ "$INSTALL_MODE" = "system" ]; then
        echo "  /usr/local/bin/almoji-menubar"
    else
        echo "  almoji-menubar"
    fi
    echo ""
    echo "Features:"
    echo "  • Adds a 😊 icon to your menubar"
    echo "  • Press Cmd+Shift+E to search for emojis"
    echo "  • Automatically copies selected emojis to clipboard"
    echo ""
    echo "Tip: Add almoji-menubar to Login Items in System Settings"
    echo "     for automatic startup on login."
    echo ""
fi

# Exit with success if CLI installed, even if menubar failed
if [ "$CLI_SUCCESS" = true ]; then
    echo "Installation completed successfully! 🎉"
    exit 0
else
    echo "Installation failed. Please check the error messages above."
    exit 1
fi
