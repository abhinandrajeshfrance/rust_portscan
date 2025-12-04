# Setup & Build Guide

This guide provides step-by-step instructions for setting up and building the Rust Port Scanner on different platforms.

## Prerequisites

### All Platforms
- **Git** - for cloning the repository  
- **Rust 1.56+** - the Rust programming language and Cargo package manager

### Windows (Additional)
- **Visual Studio Build Tools 2017+** with C++ support, OR
- **Visual Studio 2017+** with C++ desktop development, OR  
- **MinGW-w64** for GNU-based compilation

### Linux/macOS
- **GCC** or **Clang** toolchain (usually pre-installed)

## Installation Steps

### Step 1: Install Rust

#### On Windows:
1. Download the installer from https://www.rust-lang.org/tools/install
2. Run the downloaded `.exe` file
3. Follow the installation wizard (use default settings)
4. Restart your terminal

#### On Linux/macOS:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Step 2: Verify Installation

```bash
rustc --version
cargo --version
```

You should see version numbers for both commands (e.g., `rustc 1.91.1` and `cargo 1.91.1`).

### Step 3: Install Build Tools

#### Windows (Choose ONE):

**Option A: Visual Studio Build Tools (Recommended for Rust)**
1. Download from: https://aka.ms/vs/17/release/vs_BuildTools.exe
2. Run the installer
3. Select "Desktop development with C++" workload
4. Complete the installation (~2-5 GB)
5. Restart your computer

**Option B: MinGW-w64 (Lightweight)**
1. Download from: https://www.mingw-w64.org/
2. Install to `C:\mingw-w64`
3. Add `C:\mingw-w64\bin` to your Windows PATH environment variable
4. Verify: Run `gcc --version` in a new terminal

#### Linux:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora
sudo dnf groupinstall "Development Tools"

# macOS (via Homebrew)
brew install llvm
```

### Step 4: Clone the Repository

```bash
git clone https://github.com/abhinandrajeshfrance/rust_portscan.git
cd rust_portscan
```

### Step 5: Build the Project

Navigate to the `portscan` directory:

```bash
cd portscan
```

Then build:

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (slower compilation, optimized runtime)
cargo build --release
```

**On Windows**, you can also use the provided batch file:
```cmd
cd ..
build.bat
cd portscan
```

**On Linux/macOS**:
```bash
cd ..
chmod +x build.sh
./build.sh
```

### Step 6: Verify Build Success

After building, you should see:
```
Finished `release` profile [optimized] target(s) in X.XXs
```

The compiled binary will be at:
- **Windows:** `target/release/portscan.exe`
- **Linux/macOS:** `target/release/portscan`

## Running the Scanner

### Using Cargo (Easiest)

From the `portscan` directory:

```bash
# Scan localhost ports 1-1024 (default)
cargo run --release -- 127.0.0.1

# Scan specific port range with custom workers
cargo run --release -- 192.168.1.1 --start 1 --end 10000 --workers 200

# With custom timeout
cargo run --release -- scanme.nmap.org --timeout-ms 500
```

### Using Compiled Binary

After building, run the binary directly:

```bash
# Windows
.\target\release\portscan.exe 127.0.0.1 --start 1 --end 1024

# Linux/macOS
./target/release/portscan 127.0.0.1 --start 1 --end 1024
```

## Troubleshooting Build Issues

### "linker `link.exe` not found" (Windows MSVC)
**Solution:** Install Visual Studio Build Tools as described in Step 3 (Option A)

### "linker `x86_64-w64-mingw32-gcc` not found" (Windows GNU)
**Solution:** Install MinGW-w64 as described in Step 3 (Option B), or use MSVC instead

### "command not found: rustc"
**Solution:** Rust was not added to PATH. Restart your terminal or manually add:
- Windows: `%USERPROFILE%\.cargo\bin` to your PATH
- Linux/macOS: Add `. $HOME/.cargo/env` to your `.bashrc` or `.zshrc`

### "failed to parse manifest" in Cargo.toml
**Solution:** Ensure you're in the `portscan` directory (where `Cargo.toml` is located)

### Build takes very long
**Normal behavior.** First builds compile all dependencies. Subsequent builds are faster.
If it exceeds 10+ minutes on the first build, check your internet connection.

### Permission denied on Linux/macOS
```bash
chmod +x build.sh
chmod +x target/release/portscan
```

## Development & Testing

### Run with Debug Output

```bash
RUST_BACKTRACE=1 cargo run -- 127.0.0.1
```

### Run Tests (if implemented)
```bash
cargo test
```

### Check Code for Issues
```bash
cargo clippy
```

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Official Rust documentation
- [Cargo Guide](https://doc.rust-lang.org/cargo/) - Package manager guide
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive Rust tutorials

---

**Still having issues?** Check the project's README for additional help, or open an issue on GitHub.
