# Rust Port Scanner

A simple TCP port scanner written in Rust for **educational** and **defensive** purposes only.

## Features

- **Multi-threaded scanning** – Scan multiple ports in parallel for speed
- **Configurable timeout** – Set connection timeout per your network conditions
- **Flexible port ranges** – Scan specific ranges of TCP ports (1-65535)
- **Worker control** – Adjust thread count for optimal performance
- **Clean CLI** – Easy-to-use command-line interface with built-in help
- **Responsible design** – Built-in warnings and validation for safe use

## Installation

### Prerequisites

- **Rust 1.56+** and **Cargo** – Install from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/abhinandrajeshfrance/rust_portscan.git
cd rust_portscan

# Build the project
cd portscan
cargo build --release
```

The binary will be at `target/release/portscan` (or `portscan.exe` on Windows).

## Usage

### Basic Examples

**Scan common ports on localhost (default: 1–1024):**
```bash
cargo run -- 127.0.0.1
```

**Scan a specific port range:**
```bash
cargo run -- 192.168.1.1 --start 20 --end 443
```

**Scan with more workers (faster, but heavier on network):**
```bash
cargo run -- scanme.nmap.org --start 1 --end 10000 --workers 500
```

**Scan with custom timeout (500 ms per connection):**
```bash
cargo run -- 10.0.0.1 --start 1 --end 65535 --timeout-ms 500
```

### Full CLI Options

```
Usage: portscan <TARGET> [OPTIONS]

Arguments:
  <TARGET>  Target IPv4/hostname (e.g. 127.0.0.1 or scanme.nmap.org)

Options:
  -s, --start <START>              Start of TCP port range (inclusive) [default: 1]
  -e, --end <END>                  End of TCP port range (inclusive) [default: 1024]
  -w, --workers <WORKERS>          Max number of worker threads [default: 100]
      --timeout-ms <TIMEOUT_MS>    Connection timeout in milliseconds [default: 300]
  -h, --help                        Print help
  -V, --version                     Print version
```

### Running the Compiled Binary

```bash
# From the portscan directory after building
./target/release/portscan 127.0.0.1 --start 1 --end 1024

# Or directly with cargo
cargo run --release -- 127.0.0.1 --start 1 --end 1024
```

## Project Structure

```
rust_portscan/
├── portscan/
│   ├── Cargo.toml              # Project metadata and dependencies
│   ├── Cargo.lock              # Locked dependency versions
│   └── src/
│       └── main.rs             # Main scanner implementation
├── README.md                   # This file
├── .gitignore                  # Git ignore patterns
└── .git/                       # Git repository
```

## How It Works

1. **CLI Parsing** – Uses `clap` to parse command-line arguments
2. **Input Validation** – Checks that port range is valid and logical
3. **Thread Pool** – Divides the port range evenly among worker threads
4. **Scanning** – Each worker connects to assigned ports with a timeout
5. **Result Collection** – Thread-safe `Arc<Mutex<Vec>>` collects open ports
6. **Output** – Displays open ports and summary

### Thread Safety

- Uses `Arc<Mutex<Vec<u16>>>` to safely collect open ports from multiple threads
- Each thread gets its own port range to avoid contention
- All threads are joined before final output

## Responsible Use Agreement

⚠️ **IMPORTANT**: This tool is for **authorized testing only**.

- **Only scan systems you own** or where you have **explicit written permission**
- **Do not use on third-party networks** without authorization
- **Port scanning may be illegal** if performed without permission
- **The authors and your organization accept no responsibility** for misuse

## Performance Tips

- **Localhost scanning** – Very fast (sub-second for default 1–1024)
- **Network scanning** – Adjust `--workers` and `--timeout-ms` based on your network:
  - Low latency network → increase workers to 200–500
  - High latency network → increase timeout to 500–1000 ms
  - Congested network → reduce workers to 50–100
- **Full port scan** – 65535 ports with 200 workers and 300 ms timeout typically takes ~60–90 seconds

## Example Scenarios

### Scenario 1: Scan Your Local Machine
```bash
cargo run -- 127.0.0.1 --start 1 --end 5000
```

### Scenario 2: Scan a Public Test Service
```bash
# scanme.nmap.org is explicitly designed for this
cargo run -- scanme.nmap.org --start 1 --end 1024 --workers 100
```

### Scenario 3: Aggressive Scan with High Worker Count
```bash
cargo run -- 192.168.1.50 --start 1 --end 10000 --workers 500 --timeout-ms 200
```

### Scenario 4: Slow Scan Over High-Latency Network
```bash
cargo run -- 10.0.0.100 --start 80 --end 443 --workers 20 --timeout-ms 1000
```

## Dependencies

- **clap 4** – Fast command-line argument parsing with derive macros
- **std::net::TcpStream** – Built-in Rust networking
- **std::thread** – Built-in Rust threading

## Troubleshooting

### "Invalid target address"
Ensure the target is a valid IPv4 address (e.g., `127.0.0.1`, `192.168.1.1`) or hostname (e.g., `scanme.nmap.org`).

### "Start port must be <= end port"
Verify your `--start` value is less than or equal to `--end`.

### Ports always show as closed
- Check network connectivity to the target
- Increase `--timeout-ms` for distant targets
- Ensure the firewall isn't blocking your source

### Slow scans
- Increase `--workers` to parallel more connections
- Decrease `--timeout-ms` (risky on slow networks)
- Test with a smaller port range first

## Contributing

Contributions and feedback are welcome. Please ensure:
- Code follows Rust idioms and best practices
- Responsible use warnings are preserved
- New features don't compromise security

## License

MIT License – See LICENSE file (if present) for details.

## Authors

- **Abhinand Rajesh** – ESILV, class XYZ

## Disclaimer

This software is provided as-is for educational purposes. Unauthorized network scanning is illegal in many jurisdictions. Always obtain written permission before scanning any network you do not own. The authors assume no liability for misuse.

---

*Last updated: December 4, 2025*
