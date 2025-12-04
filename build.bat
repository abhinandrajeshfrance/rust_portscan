@echo off
REM Build and setup script for Rust Port Scanner on Windows
REM This script handles building the project

echo [*] Rust Port Scanner - Build Script for Windows
echo [*] Checking for Rust installation...

where cargo >nul 2>nul
if errorlevel 1 (
    echo [-] Cargo not found. Please install Rust from https://www.rust-lang.org/tools/install
    pause
    exit /b 1
)

for /f "tokens=*" %%i in ('rustc --version') do set RUST_VERSION=%%i
for /f "tokens=*" %%i in ('cargo --version') do set CARGO_VERSION=%%i

echo [+] %RUST_VERSION%
echo [+] %CARGO_VERSION%

echo.
cd /d "%~dp0portscan"

echo [*] Building project...
cargo build --release

if errorlevel 1 (
    echo [-] Build failed. Please check dependencies.
    pause
    exit /b 1
)

echo [+] Build successful!
echo [+] Binary location: portscan\target\release\portscan.exe
echo.
echo [*] To run the scanner:
echo     cargo run --release -- 127.0.0.1 --start 1 --end 1024
echo.
pause
