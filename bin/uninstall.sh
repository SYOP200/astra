#!/bin/sh

set -e

echo "Unloading Astra..."
echo "Removing packages..."
echo "Removing dependencies..."

INSTALL_DIR="/usr/local/bin"

echo "Disabling Astra..."
echo "Removing directory..."

sudo rm -rf "$INSTALL_DIR/astra"

echo "Astra uninstalled successfully."
echo "See you next time!"
