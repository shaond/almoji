#!/bin/bash
# Build script for Almoji Alfred Workflow

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
echo "You can now double-click Almoji-Workflow.alfredworkflow to install it in Alfred"
