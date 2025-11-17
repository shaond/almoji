#!/bin/bash
# Almoji Installation Script
# Builds and installs the Almoji emoji search CLI tool

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default installation directory
INSTALL_DIR="/usr/local/bin"

# Print colored output
print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_header() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}        Almoji Installer 🎉            ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC}   Fast Emoji Search for macOS        ${BLUE}║${NC}"
    echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
    echo ""
}

# Check if Rust is installed
check_rust() {
    print_info "Checking for Rust installation..."
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not found!"
        echo ""
        echo "Please install Rust from: https://rustup.rs/"
        echo "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    print_success "Rust found: $(cargo --version)"
}

# Build the project
build_almoji() {
    print_info "Building Almoji in release mode..."
    if cargo build --release; then
        print_success "Build completed successfully!"
    else
        print_error "Build failed!"
        exit 1
    fi
}

# Run tests
run_tests() {
    print_info "Running tests..."
    if cargo test --release --quiet; then
        print_success "All tests passed!"
    else
        print_warning "Some tests failed, but continuing installation..."
    fi
}

# Install the binary
install_binary() {
    local binary_path="./target/release/almoji"

    if [ ! -f "$binary_path" ]; then
        print_error "Binary not found at $binary_path"
        exit 1
    fi

    print_info "Installing almoji to $INSTALL_DIR..."

    # Check if we need sudo
    if [ -w "$INSTALL_DIR" ]; then
        cp "$binary_path" "$INSTALL_DIR/almoji"
    else
        print_warning "Need sudo access to install to $INSTALL_DIR"
        sudo cp "$binary_path" "$INSTALL_DIR/almoji"
        sudo chmod +x "$INSTALL_DIR/almoji"
    fi

    print_success "Almoji installed to $INSTALL_DIR/almoji"
}

# Verify installation
verify_installation() {
    print_info "Verifying installation..."

    if command -v almoji &> /dev/null; then
        print_success "Almoji is installed and available in PATH"
        echo ""
        echo -e "${GREEN}Version:${NC} $(almoji --version)"
        echo ""

        # Test with a simple query
        print_info "Testing with 'almoji fire'..."
        almoji fire --limit 1
        echo ""
    else
        print_error "Almoji not found in PATH!"
        print_warning "You may need to add $INSTALL_DIR to your PATH"
        exit 1
    fi
}

# Install Alfred workflow
install_alfred_workflow() {
    if [ -f "Almoji-Workflow.alfredworkflow" ]; then
        echo ""
        read -p "Would you like to install the Alfred workflow? (y/N) " -n 1 -r
        echo ""

        if [[ $REPLY =~ ^[Yy]$ ]]; then
            print_info "Opening Alfred workflow..."
            open "Almoji-Workflow.alfredworkflow"
            print_success "Alfred should prompt you to import the workflow"
            echo ""
            print_info "After importing, use ';' in Alfred to search for emojis"
        fi
    fi
}

# Show usage examples
show_examples() {
    echo ""
    echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║${NC}           Installation Complete!      ${GREEN}║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BLUE}Usage examples:${NC}"
    echo ""
    echo "  almoji fire              # Search for fire emoji"
    echo "  almoji heart             # Search for heart emojis"
    echo "  almoji --limit 5 happy   # Limit results to 5"
    echo "  almoji --skin-tone dark wave    # With skin tone modifier"
    echo "  almoji --gender female shrug    # With gender variant"
    echo ""
    echo -e "${BLUE}Options:${NC}"
    echo ""
    echo "  -l, --limit <N>          Maximum number of results"
    echo "  -s, --skin-tone <TONE>   Skin tone (light, medium-light, medium, medium-dark, dark)"
    echo "  -g, --gender <GENDER>    Gender variant (male, female, neutral)"
    echo "  -h, --help               Show help"
    echo "  -V, --version            Show version"
    echo ""
    echo -e "${BLUE}For Alfred workflow:${NC}"
    echo "  Type ';' in Alfred followed by your search term"
    echo ""
}

# Uninstall function
uninstall() {
    print_info "Uninstalling Almoji..."

    if [ -f "$INSTALL_DIR/almoji" ]; then
        if [ -w "$INSTALL_DIR" ]; then
            rm "$INSTALL_DIR/almoji"
        else
            sudo rm "$INSTALL_DIR/almoji"
        fi
        print_success "Almoji has been uninstalled"
    else
        print_warning "Almoji not found at $INSTALL_DIR/almoji"
    fi

    exit 0
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --uninstall)
            uninstall
            ;;
        --prefix)
            INSTALL_DIR="$2"
            shift 2
            ;;
        --skip-tests)
            SKIP_TESTS=1
            shift
            ;;
        --help)
            echo "Almoji Installation Script"
            echo ""
            echo "Usage: ./install.sh [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --prefix DIR      Install to custom directory (default: /usr/local/bin)"
            echo "  --skip-tests      Skip running tests"
            echo "  --uninstall       Uninstall Almoji"
            echo "  --help            Show this help message"
            echo ""
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Run './install.sh --help' for usage"
            exit 1
            ;;
    esac
done

# Main installation process
main() {
    print_header
    check_rust
    build_almoji

    if [ -z "$SKIP_TESTS" ]; then
        run_tests
    fi

    install_binary
    verify_installation
    install_alfred_workflow
    show_examples
}

# Run main function
main
