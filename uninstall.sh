#!/usr/bin/env bash
set -euo pipefail

BIN_DIR="${HOME}/.local/bin"
INSTALL_PATH="${BIN_DIR}/mneme-g"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/mneme-guardian"
CACHE_DIR="${XDC_CACHE_HOME:-$HOME/.cache}/mneme-guardian"

if [[ -f "$INSTALL_PATH" ]]; then
    rm "$INSTALL_PATH"
    echo "✓ Removed binary: ${INSTALL_PATH}"
else
    echo "ℹ Binary not found: ${INSTALL_PATH}"
fi

if [[ -d "$CONFIG_DIR" ]]; then
    rm -rf "$CONFIG_DIR"
    echo "✓ Removed config: ${CONFIG_DIR}"
fi

if [[ -d "$CACHE_DIR" ]]; then
    rm -rf "$CACHE_DIR"
    echo "✓ Removed cache: ${CACHE_DIR}"
fi

echo "✓ mneme-guardian uninstalled."
