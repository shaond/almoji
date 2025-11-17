.PHONY: all build install uninstall test clean help alfred

# Installation directory
PREFIX ?= /usr/local
INSTALL_DIR = $(PREFIX)/bin

# Binary name
BINARY = almoji
TARGET_DIR = target/release

all: build

# Build the project in release mode
build:
	@echo "Building Almoji..."
	@cargo build --release
	@echo "✓ Build complete: $(TARGET_DIR)/$(BINARY)"

# Install the binary
install: build
	@echo "Installing $(BINARY) to $(INSTALL_DIR)..."
	@install -d $(INSTALL_DIR)
	@install -m 755 $(TARGET_DIR)/$(BINARY) $(INSTALL_DIR)/$(BINARY)
	@echo "✓ Installed $(BINARY) to $(INSTALL_DIR)/$(BINARY)"
	@echo ""
	@echo "Try it out: $(BINARY) fire"

# Uninstall the binary
uninstall:
	@echo "Uninstalling $(BINARY) from $(INSTALL_DIR)..."
	@rm -f $(INSTALL_DIR)/$(BINARY)
	@echo "✓ Uninstalled $(BINARY)"

# Run tests
test:
	@echo "Running tests..."
	@cargo test
	@echo "✓ All tests passed"

# Run tests with coverage
test-verbose:
	@cargo test -- --nocapture

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@echo "✓ Clean complete"

# Install Alfred workflow
alfred:
	@if [ -f "Almoji-Workflow.alfredworkflow" ]; then \
		echo "Opening Alfred workflow..."; \
		open Almoji-Workflow.alfredworkflow; \
		echo "✓ Alfred should prompt you to import the workflow"; \
	else \
		echo "✗ Alfred workflow not found"; \
		exit 1; \
	fi

# Development build (debug mode)
dev:
	@echo "Building in debug mode..."
	@cargo build
	@echo "✓ Debug build complete"

# Run the binary with arguments (usage: make run ARGS="fire")
run: build
	@$(TARGET_DIR)/$(BINARY) $(ARGS)

# Check code formatting
fmt:
	@cargo fmt --check

# Format code
fmt-fix:
	@cargo fmt

# Run clippy lints
lint:
	@cargo clippy -- -D warnings

# Check everything (fmt, lint, test)
check: fmt lint test
	@echo "✓ All checks passed"

# Show help
help:
	@echo "Almoji Build and Install Targets"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  all          Build the project (default)"
	@echo "  build        Build in release mode"
	@echo "  install      Build and install to $(INSTALL_DIR)"
	@echo "  uninstall    Remove installed binary"
	@echo "  test         Run tests"
	@echo "  clean        Remove build artifacts"
	@echo "  alfred       Install Alfred workflow"
	@echo "  dev          Build in debug mode"
	@echo "  run          Build and run (use ARGS='...' for arguments)"
	@echo "  fmt          Check code formatting"
	@echo "  fmt-fix      Format code"
	@echo "  lint         Run clippy lints"
	@echo "  check        Run all checks (fmt, lint, test)"
	@echo "  help         Show this help message"
	@echo ""
	@echo "Options:"
	@echo "  PREFIX       Installation prefix (default: /usr/local)"
	@echo ""
	@echo "Examples:"
	@echo "  make install              # Install to /usr/local/bin"
	@echo "  make install PREFIX=~/.local  # Install to ~/.local/bin"
	@echo "  make run ARGS='heart'     # Run with 'heart' argument"
