#!/usr/bin/env bash
set -euo pipefail

VERSION="0.1.0"
BIN_DIR="${HOME}/.local/bin"
INSTALL_PATH="${BIN_DIR}/mneme-g"

echo "Installing mneme-guardian v${VERSION}..."

mkdir -p "$BIN_DIR"
cp "$(dirname "$0")/bin/mneme-g" "$INSTALL_PATH"
chmod +x "$INSTALL_PATH"

echo "✓ Installed to ${INSTALL_PATH}"

# Add to PATH if needed
if [[ ":$PATH:" != *":${BIN_DIR}:"* ]]; then
    echo ""
    echo "ℹ Add ${BIN_DIR} to your PATH:"
    echo "  echo 'export PATH=\"\$PATH:${BIN_DIR}\"' >> ~/.bashrc"
    echo "  source ~/.bashrc"
fi

echo ""
echo "Quick start:"
echo "  cd your-project"
echo "  mneme-g init      # Create config"
echo "  mneme-g install   # Install pre-commit hook"
echo "  mneme-g run        # Review staged files"
