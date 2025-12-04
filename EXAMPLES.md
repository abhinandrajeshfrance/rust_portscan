# Examples

Collection of practical examples for using the Rust Port Scanner.

## Basic Usage

### Example 1: Scan Localhost (Default)

Scan your local machine for open ports 1-1024:

```bash
cargo run --release -- 127.0.0.1
```

Expected output:
```
[*] Scanning 127.0.0.1 from ports 1 to 1024 with 100 workers...
[*] Timeout: 300 ms

[!] Only scan systems you own or have explicit permission to test.

[+] Port 22 open
[+] Port 3306 open
[+] Port 8080 open

[*] Scan complete.
[+] Open ports found:
    - 22
    - 3306
    - 8080
```

### Example 2: Scan with Custom Range

Scan a specific range of ports:

```bash
cargo run --release -- 192.168.1.100 --start 80 --end 443
```

This scans ports 80, 81, 82... up to 443.

### Example 3: Aggressive Scan (Many Workers)

For high-speed scanning on a local network or with good connectivity:

```bash
cargo run --release -- 10.0.0.50 --start 1 --end 10000 --workers 500 --timeout-ms 200
```

**Note:** Increase `--workers` and decrease `--timeout-ms` only on low-latency networks!

### Example 4: Conservative Scan (High-Latency Networks)

For scanning over the internet or on slow networks:

```bash
cargo run --release -- scanme.nmap.org --start 1 --end 5000 --workers 50 --timeout-ms 1000
```

Higher timeout and fewer workers = more reliable results on slow connections.

### Example 5: Full Port Range Scan

Scan all 65535 TCP ports (this will take time - typically 30-90 seconds depending on the target):

```bash
cargo run --release -- 192.168.1.1 --start 1 --end 65535 --workers 200
```

### Example 6: Common Ports Only

Scan only the most commonly used ports (web, SSH, DNS, etc.):

```bash
cargo run --release -- example.com --start 20 --end 443 --workers 100
```

Common ports:
- **21:** FTP
- **22:** SSH
- **25:** SMTP
- **53:** DNS
- **80:** HTTP
- **110:** POP3
- **143:** IMAP
- **443:** HTTPS
- **3306:** MySQL
- **5432:** PostgreSQL
- **6379:** Redis
- **8080:** HTTP (alternate)

## Advanced Usage

### Example 7: Using the Compiled Binary Directly

After building, you can run the binary directly without cargo:

**Windows:**
```cmd
.\portscan\target\release\portscan.exe 127.0.0.1
```

**Linux/macOS:**
```bash
./portscan/target/release/portscan 127.0.0.1
```

### Example 8: Redirect Output to File

Save scan results to a file:

**Windows:**
```cmd
cargo run --release -- 192.168.1.1 > scan_results.txt 2>&1
```

**Linux/macOS:**
```bash
cargo run --release -- 192.168.1.1 | tee scan_results.txt
```

### Example 9: Scan Multiple Targets

Create a simple shell script to scan multiple hosts:

**Linux/macOS (`scan_network.sh`):**
```bash
#!/bin/bash
for i in {1..10}; do
    echo "[*] Scanning 192.168.1.$i..."
    cargo run --release -- 192.168.1.$i --start 1 --end 1024
done
```

Run with:
```bash
chmod +x scan_network.sh
./scan_network.sh
```

### Example 10: Compare Results with nmap

Test against scanme.nmap.org (public test server):

```bash
# Our scanner (first 1000 ports)
cargo run --release -- scanme.nmap.org --start 1 --end 1000 --workers 100

# Compare with nmap (if installed)
nmap -p1-1000 scanme.nmap.org
```

## Performance Tuning

### For Fast Networks (LAN):
```bash
cargo run --release -- target --workers 500 --timeout-ms 100
```

### For Medium Networks:
```bash
cargo run --release -- target --workers 200 --timeout-ms 300
```

### For Slow Networks (WAN):
```bash
cargo run --release -- target --workers 50 --timeout-ms 1000
```

### For Very Slow Networks:
```bash
cargo run --release -- target --workers 10 --timeout-ms 2000
```

## Help & Options

View all available options:

```bash
cargo run --release -- --help
```

Output:
```
Simple TCP Port Scanner

Usage: portscan <TARGET> [OPTIONS]

Arguments:
  <TARGET>  Target IPv4/hostname (e.g., 127.0.0.1 or scanme.nmap.org)

Options:
  -s, --start <START>          Start of TCP port range (default: 1)
  -e, --end <END>              End of TCP port range (default: 1024)
  -w, --workers <WORKERS>      Max number of worker threads (default: 100)
      --timeout-ms <MS>        Connection timeout in milliseconds (default: 300)
  -h, --help                   Print this help message
```

## Responsible Use Reminders

⚠️ **ALWAYS remember:**

1. **Get Permission First** – Only scan systems you own or have explicit written authorization to test
2. **Respect Firewalls** – Don't bypass security controls
3. **Be Aware of Laws** – Port scanning may be illegal in your jurisdiction without permission
4. **Use Reasonably** – Excessive scanning can disrupt network operations
5. **Document Authorization** – Keep records of permission for scanning

## Example Output Interpretation

### Open Port
```
[+] Port 22 open
```
Means a TCP connection was successfully established to port 22 on the target.

### Port Doesn't Appear
A port is closed if no `[+]` line is shown for it.

### Completed Scan
```
[*] Scan complete.
[+] Open ports found:
    - 22
    - 80
    - 443
```
Lists all discovered open ports in sorted order.

## Tips & Tricks

1. **Test Locally First** – Before scanning remote targets, test on `127.0.0.1`
2. **Use Hostnames** – You can use domain names instead of IPs: `cargo run -- example.com`
3. **Increase Workers Gradually** – Start with 100 workers, increase if needed (up to 1000)
4. **Monitor System Resources** – High worker counts use significant system resources
5. **Chain Multiple Scans** – Use scripts to scan different port ranges or targets sequentially

---

For more information, see the main [README.md](README.md) and [SETUP.md](SETUP.md).
