# Makefile for clipboard_manager

# Variables
CARGO := cargo
TARGET_DIR := target/release
BIN_NAME := clipboard_manager
INSTALL_DIR := /usr/local/bin

# Phony targets
.PHONY: all release install clean uninstall help

# Default target
all: release

# Build release version
release:
	@echo "Building release version..."
	$(CARGO) build --release

# Install binary to system
install: release
	@echo "Installing $(BIN_NAME) to $(INSTALL_DIR)..."
	@if [ ! -d "$(INSTALL_DIR)" ]; then \
		echo "Directory $(INSTALL_DIR) does not exist. Creating it..."; \
		sudo mkdir -p $(INSTALL_DIR); \
	fi
	sudo cp $(TARGET_DIR)/$(BIN_NAME) $(INSTALL_DIR)/$(BIN_NAME)
	sudo chmod +x $(INSTALL_DIR)/$(BIN_NAME)
	@echo "Installation complete. You can now run '$(BIN_NAME)'."

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean

# Uninstall binary
uninstall:
	@echo "Uninstalling $(BIN_NAME)..."
	sudo rm -f $(INSTALL_DIR)/$(BIN_NAME)
	@echo "Uninstallation complete."

# Show help
help:
	@echo "Usage:"
	@echo "  make         Build release version"
	@echo "  make release Build release version"
	@echo "  make install Install to $(INSTALL_DIR) (requires sudo)"
	@echo "  make clean   Clean build artifacts"
	@echo "  make uninstall Remove installed binary"
	@echo "  make help    Show this help message"
