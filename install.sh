#!/bin/sh
# Hype-RS Installation Script
# Usage: curl -fsSL https://raw.githubusercontent.com/twilson63/hype-rs/master/install.sh | sh
# Or: curl -fsSL https://raw.githubusercontent.com/twilson63/hype-rs/master/install.sh | sh -s -- --version v0.2.0

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="twilson63/hype-rs"
INSTALL_DIR="${HYPE_INSTALL_DIR:-$HOME/.hype}"
BIN_DIR="$INSTALL_DIR/bin"
VERSION="${1:-latest}"

# Helper functions
info() {
    printf "${GREEN}==>${NC} %s\n" "$1"
}

warn() {
    printf "${YELLOW}Warning:${NC} %s\n" "$1"
}

error() {
    printf "${RED}Error:${NC} %s\n" "$1" >&2
    exit 1
}

# Detect platform
detect_platform() {
    local os
    local arch
    local platform
    
    os="$(uname -s)"
    arch="$(uname -m)"
    
    case "$os" in
        Linux)
            case "$arch" in
                x86_64)
                    # Check if musl or gnu
                    if ldd /bin/ls 2>&1 | grep -q musl; then
                        platform="x86_64-unknown-linux-musl"
                    else
                        platform="x86_64-unknown-linux-gnu"
                    fi
                    ;;
                aarch64|arm64)
                    platform="aarch64-unknown-linux-gnu"
                    ;;
                *)
                    error "Unsupported architecture: $arch"
                    ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64)
                    platform="x86_64-apple-darwin"
                    ;;
                arm64|aarch64)
                    platform="aarch64-apple-darwin"
                    ;;
                *)
                    error "Unsupported architecture: $arch"
                    ;;
            esac
            ;;
        *)
            error "Unsupported operating system: $os"
            ;;
    esac
    
    echo "$platform"
}

# Get latest release version
get_latest_version() {
    local url="https://api.github.com/repos/$REPO/releases/latest"
    
    if command -v curl > /dev/null 2>&1; then
        curl -s "$url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    elif command -v wget > /dev/null 2>&1; then
        wget -qO- "$url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    else
        error "curl or wget is required"
    fi
}

# Download file
download() {
    local url="$1"
    local output="$2"
    
    if command -v curl > /dev/null 2>&1; then
        curl -fsSL "$url" -o "$output"
    elif command -v wget > /dev/null 2>&1; then
        wget -q "$url" -O "$output"
    else
        error "curl or wget is required"
    fi
}

# Verify checksum
verify_checksum() {
    local file="$1"
    local checksum_file="$2"
    
    if command -v shasum > /dev/null 2>&1; then
        cd "$(dirname "$file")" && shasum -a 256 -c "$(basename "$checksum_file")" > /dev/null 2>&1
    elif command -v sha256sum > /dev/null 2>&1; then
        cd "$(dirname "$file")" && sha256sum -c "$(basename "$checksum_file")" > /dev/null 2>&1
    else
        warn "shasum/sha256sum not found, skipping checksum verification"
        return 0
    fi
}

# Main installation
main() {
    info "Installing Hype-RS..."
    
    # Detect platform
    PLATFORM=$(detect_platform)
    info "Detected platform: $PLATFORM"
    
    # Get version
    if [ "$VERSION" = "latest" ]; then
        VERSION=$(get_latest_version)
        if [ -z "$VERSION" ]; then
            error "Failed to determine latest version"
        fi
        info "Latest version: $VERSION"
    fi
    
    # Construct download URL
    ARCHIVE="hype-${PLATFORM}.tar.gz"
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$ARCHIVE"
    CHECKSUM_URL="$DOWNLOAD_URL.sha256"
    
    # Create temporary directory
    TMP_DIR=$(mktemp -d)
    trap 'rm -rf "$TMP_DIR"' EXIT
    
    # Download archive
    info "Downloading from $DOWNLOAD_URL..."
    download "$DOWNLOAD_URL" "$TMP_DIR/$ARCHIVE" || error "Failed to download archive"
    
    # Download and verify checksum
    info "Verifying checksum..."
    download "$CHECKSUM_URL" "$TMP_DIR/$ARCHIVE.sha256" || warn "Failed to download checksum, skipping verification"
    if [ -f "$TMP_DIR/$ARCHIVE.sha256" ]; then
        verify_checksum "$TMP_DIR/$ARCHIVE" "$TMP_DIR/$ARCHIVE.sha256" || error "Checksum verification failed"
    fi
    
    # Extract archive
    info "Extracting archive..."
    mkdir -p "$BIN_DIR"
    tar xzf "$TMP_DIR/$ARCHIVE" -C "$TMP_DIR"
    
    # Install binary
    info "Installing to $BIN_DIR/hype..."
    mv "$TMP_DIR/hype" "$BIN_DIR/hype"
    chmod +x "$BIN_DIR/hype"
    
    # Success message
    echo ""
    info "✓ Installation complete!"
    echo ""
    echo "Hype-RS has been installed to: $BIN_DIR/hype"
    echo ""
    
    # Check if binary is in PATH
    if echo "$PATH" | grep -q "$BIN_DIR"; then
        info "You can now run: hype --version"
    else
        warn "The install directory is not in your PATH."
        echo ""
        echo "To add it to your PATH, run:"
        echo ""
        echo "    export PATH=\"$BIN_DIR:\$PATH\""
        echo ""
        echo "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.) to make it permanent:"
        echo ""
        if [ -f "$HOME/.zshrc" ]; then
            echo "    echo 'export PATH=\"$BIN_DIR:\$PATH\"' >> ~/.zshrc"
        elif [ -f "$HOME/.bashrc" ]; then
            echo "    echo 'export PATH=\"$BIN_DIR:\$PATH\"' >> ~/.bashrc"
        else
            echo "    echo 'export PATH=\"$BIN_DIR:\$PATH\"' >> ~/.profile"
        fi
        echo ""
    fi
    
    # Verify installation
    if "$BIN_DIR/hype" --version > /dev/null 2>&1; then
        INSTALLED_VERSION=$("$BIN_DIR/hype" --version 2>&1 | head -n1)
        info "Installed: $INSTALLED_VERSION"
    fi
}

# Parse arguments
while [ $# -gt 0 ]; do
    case "$1" in
        --version)
            shift
            VERSION="$1"
            ;;
        *)
            error "Unknown option: $1"
            ;;
    esac
    shift
done

# Run installation
main
