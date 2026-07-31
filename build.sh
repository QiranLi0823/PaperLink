#!/bin/bash
# Paper Studio Phase 0 - Build Script
# This script builds the WASM module and prepares the web application

set -e

echo "=== Paper Studio Phase 0 Build ==="

# Check prerequisites
command -v cargo >/dev/null 2>&1 || { echo "Error: Rust/Cargo not installed"; exit 1; }
command -v wasm-pack >/dev/null 2>&1 || {
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
}

# Build WASM module
echo ""
echo "Building WASM module..."
cd paper-core
wasm-pack build --target web --out-dir ../web/pkg
cd ..

# Verify build
if [ -f "web/pkg/paper_core.js" ]; then
    echo ""
    echo "=== Build Successful ==="
    echo "WASM module built to: web/pkg/"
    ls -la web/pkg/
else
    echo ""
    echo "=== Build Failed ==="
    echo "WASM module not found. The web app will use JS fallback parser."
fi

echo ""
echo "To run the application:"
echo "  cd web && python3 -m http.server 8080"
echo "  Then open http://localhost:8080"
