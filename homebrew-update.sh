#!/bin/bash
# Homebrew Tap Update Script for hype-rs v0.4.2
# This script automates the process of updating the Homebrew formula

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
HOMEBREW_TAP_REPO="git@github.com:twilson63/homebrew-tap.git"
HOMEBREW_TAP_DIR="${HOME}/homebrew-tap"
HYPE_VERSION="0.4.2"
HYPE_SHA256="9fc16b47f92638c9732557f62565c519bb63056b761888aa43df354203c87728"
FORMULA_NAME="hype.rb"
FORMULA_PATH="Formula/hype.rb"

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

success() {
    printf "${GREEN}✓${NC} %s\n" "$1"
}

section() {
    printf "\n${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
    printf "${BLUE}%s${NC}\n" "$1"
    printf "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

# Main script
main() {
    section "Homebrew Tap Update Script - hype-rs v${HYPE_VERSION}"
    
    # Step 1: Check if homebrew-tap exists locally
    info "Step 1: Checking homebrew-tap repository..."
    
    if [ ! -d "$HOMEBREW_TAP_DIR" ]; then
        info "Cloning homebrew-tap repository from GitHub..."
        git clone "$HOMEBREW_TAP_REPO" "$HOMEBREW_TAP_DIR" || error "Failed to clone homebrew-tap repository"
        success "Repository cloned successfully"
    else
        info "Repository already exists at $HOMEBREW_TAP_DIR"
        cd "$HOMEBREW_TAP_DIR"
        git pull origin master || error "Failed to pull latest changes"
        success "Repository updated"
    fi
    
    cd "$HOMEBREW_TAP_DIR"
    
    # Step 2: Create/update the formula
    section "Step 2: Creating/Updating Homebrew Formula"
    
    info "Creating formula at $FORMULA_PATH..."
    
    mkdir -p "$(dirname "$FORMULA_PATH")"
    
    cat > "$FORMULA_PATH" << 'EOF'
class Hype < Formula
  desc "A high-performance Lua scripting engine with CLI interface and package management"
  homepage "https://github.com/twilson63/hype-rs"
  url "https://github.com/twilson63/hype-rs/releases/download/v0.4.2/hype-x86_64-apple-darwin.tar.gz"
  sha256 "9fc16b47f92638c9732557f62565c519bb63056b761888aa43df354203c87728"
  license "MIT OR Apache-2.0"

  def install
    bin.install "hype"
  end

  test do
    assert_match /version/, shell_output("#{bin}/hype --version")
  end
end
EOF
    
    success "Formula created at $FORMULA_PATH"
    
    # Step 3: Verify the formula syntax
    section "Step 3: Verifying Formula Syntax"
    
    info "Checking Ruby syntax..."
    ruby -c "$FORMULA_PATH" || error "Formula has syntax errors"
    success "Formula syntax is valid"
    
    # Step 4: Git operations
    section "Step 4: Committing Changes"
    
    info "Adding formula to git..."
    git add "$FORMULA_PATH"
    
    info "Checking git status..."
    git status
    
    info "Committing changes..."
    git commit -m "chore: Bump hype to v${HYPE_VERSION}

- Update URL to v${HYPE_VERSION} release archive
- Update SHA256 checksum
- New feature: Direct file/directory fallback for module resolver
" || error "Failed to commit changes"
    
    success "Changes committed successfully"
    
    # Step 5: Final instructions
    section "Step 5: Ready to Push!"
    
    echo ""
    echo "Next steps:"
    echo ""
    echo "1. Review the changes:"
    echo "   cd $HOMEBREW_TAP_DIR"
    echo "   git log -1 --stat"
    echo ""
    echo "2. Optionally test the formula locally:"
    echo "   brew test-bot --only-formulae Formula/hype.rb"
    echo ""
    echo "3. Push to remote:"
    echo "   git push origin master"
    echo ""
    echo "4. After pushing, users can install with:"
    echo "   brew install twilson63/tap/hype"
    echo ""
    
    success "Homebrew update script completed!"
    success "Formula location: $HOMEBREW_TAP_DIR/$FORMULA_PATH"
}

# Run main function
main "$@"
