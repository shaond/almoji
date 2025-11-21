#!/bin/bash
# Build and installation script for Almoji

set -e

echo "Building Almoji..."
echo ""

# Build the Rust binary in release mode
echo "1. Compiling Rust binary..."
cargo build --release

# Copy binary to workflow directory
echo "2. Copying binary to workflow directory..."
cp target/release/almoji Almoji.alfredworkflow/

# Create the Alfred workflow package (zip file)
echo "3. Creating Alfred workflow package..."
cd Almoji.alfredworkflow
zip -r ../Almoji-Workflow.alfredworkflow * -x "*.DS_Store"
cd ..

echo ""
echo "✓ Build complete!"
echo "  - Binary: target/release/almoji"
echo "  - Workflow: Almoji-Workflow.alfredworkflow"
echo ""

# Check if we should install the binary
if [ "$1" = "--system" ] || [ "$1" = "-s" ]; then
    echo "Installing to system directory (/usr/local/bin)..."
    cargo install --path . --root /usr/local
    echo ""
    echo "✓ Almoji installed to /usr/local/bin/almoji"
elif [ "$1" = "--install" ] || [ "$1" = "-i" ]; then
    echo "Installing to user directory (~/.cargo/bin)..."
    cargo install --path .
    echo ""
    echo "✓ Almoji installed to ~/.cargo/bin/almoji"
    echo ""
    echo "Make sure ~/.cargo/bin is in your PATH"
else
    echo "To install the binary, run:"
    echo "  ./install.sh --install    # Install to ~/.cargo/bin"
    echo "  ./install.sh --system     # Install to /usr/local/bin"
fi

echo ""
echo "You can now double-click Almoji-Workflow.alfredworkflow to install it in Alfred"
