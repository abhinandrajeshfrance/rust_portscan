#!/usr/bin/env bash
# Build and setup script for Rust Port Scanner
# This script handles building the project across different platforms

set -e

echo "[*] Rust Port Scanner - Build Script"
echo "[*] OS: $(uname -s)"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "[-] Cargo not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

echo "[+] Rust $(rustc --version)"
echo "[+] Cargo $(cargo --version)"

# Navigate to project directory
cd "$(dirname "$0")/portscan"

echo ""
echo "[*] Building project..."

# Build for release
if cargo build --release; then
    echo "[+] Build successful!"
    
    # Show binary location
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
        echo "[+] Binary location: portscan\target\release\portscan.exe"
    else
        echo "[+] Binary location: portscan/target/release/portscan"
    fi
else
    echo "[-] Build failed. Please check dependencies."
    exit 1
fi

echo ""
echo "[*] To run the scanner:"
echo "    cargo run --release -- 127.0.0.1 --start 1 --end 1024"
echo ""
