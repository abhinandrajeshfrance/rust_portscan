# Video Demo Scenarios & Commands

## IMPORTANT: Build Setup First

Since your system needs C++ build tools, here are your options:

### Option A: Download Visual Studio Build Tools (Recommended)
```
1. Go to: https://visualstudio.microsoft.com/visual-cpp-build-tools/
2. Download "Build Tools for Visual Studio 2022"
3. Run the installer
4. Select workload: "Desktop development with C++"
5. Complete installation (~2-5 GB)
6. Restart your computer
7. Then run: cd portscan && cargo build --release
```

### Option B: Use Docker (Skip build entirely)
If you have Docker installed:
```bash
docker build -t portscan .
docker run portscan 127.0.0.1 --start 1 --end 1024
```

---

## DEMO SCENARIO 1: Scan Your Local Machine (Localhost)

### Command:
```bash
cd a:\git\rust_portscan\portscan
cargo run --release -- 127.0.0.1 --start 1 --end 1024
```

### What to say in video:
"We're scanning our own computer (127.0.0.1 is localhost) for open ports from 1 to 1024. Watch how it finds open ports and prints them as it goes. With 100 parallel workers, this should finish in about 5-10 seconds."

### Expected output:
```
[*] Scanning 127.0.0.1 from ports 1 to 1024 with 100 workers...
[*] Timeout: 300 ms

[!] Only scan systems you own or have explicit permission to test.

[+] Port 22 open
[+] Port 80 open
[+] Port 443 open
[+] Port 3306 open
[+] Port 5432 open

[*] Scan complete.
[+] Open ports found:
    - 22
    - 80
    - 443
    - 3306
    - 5432
```

(Actual ports depend on what services you have running)

---

## DEMO SCENARIO 2: Scan with Custom Range (Web Ports Only)

### Command:
```bash
cargo run --release -- 127.0.0.1 --start 80 --end 443 --workers 50 --timeout-ms 500
```

### Script:
"Now let's scan a narrower range - just web-related ports 80 through 443. I'm using 50 workers instead of 100, and a longer 500ms timeout to be more thorough on these specific ports."

### Expected output:
```
[*] Scanning 127.0.0.1 from ports 80 to 443 with 50 workers...
[*] Timeout: 500 ms

[!] Only scan systems you own or have explicit permission to test.

[+] Port 80 open
[+] Port 443 open

[*] Scan complete.
[+] Open ports found:
    - 80
    - 443
```

---

## DEMO SCENARIO 3: Aggressive Local Scan (Fast)

### Command:
```bash
cargo run --release -- 127.0.0.1 --start 1 --end 10000 --workers 500 --timeout-ms 100
```

### Script:
"For a larger port range on the local machine, I can be aggressive with 500 workers and a very short 100ms timeout. The scan will complete faster because I'm looking at 10,000 ports, which would take forever at default settings."

### Expected output:
```
[*] Scanning 127.0.0.1 from ports 1 to 10000 with 500 workers...
[*] Timeout: 100 ms

[!] Only scan systems you own or have explicit permission to test.

[+] Port 22 open
[+] Port 80 open
[+] Port 443 open
... (any other open ports)

[*] Scan complete.
[+] Open ports found:
    - 22
    - 80
    - 443
    - ... (others)
```

---

## DEMO SCENARIO 4: Conservative Internet Scan

### Command:
```bash
cargo run --release -- scanme.nmap.org --start 1 --end 5000 --workers 50 --timeout-ms 2000
```

### Script:
"Scanning a remote host over the internet requires different settings. I'm using only 50 workers to avoid overwhelming the network, and a 2-second timeout because internet latency is higher. This is more respectful and reliable."

### Expected output:
```
[*] Scanning scanme.nmap.org from ports 1 to 5000 with 50 workers...
[*] Timeout: 2000 ms

[!] Only scan systems you own or have explicit permission to test.

[+] Port 22 open
[+] Port 80 open
[+] Port 9929 open

[*] Scan complete.
[+] Open ports found:
    - 22
    - 80
    - 9929
```

(scanme.nmap.org is a deliberately scan-friendly target for testing)

---

## DEMO SCENARIO 5: Help Menu

### Command:
```bash
cargo run -- --help
```

### Expected output:
```
Simple TCP Port Scanner

Usage: portscan <TARGET> [OPTIONS]

Arguments:
  <TARGET>  Target IPv4/hostname (e.g., 127.0.0.1 or scanme.nmap.org)

Options:
  -s, --start <START>       Start of TCP port range (default: 1)
  -e, --end <END>           End of TCP port range (default: 1024)
  -w, --workers <WORKERS>   Max number of worker threads (default: 100)
  --timeout-ms <MS>        Connection timeout in milliseconds (default: 300)
  -h, --help                Print this help message
```

### Script:
"The program has built-in help showing all available options and their defaults."

---

## DEMO SCENARIO 6: Error Handling Examples

### Invalid port range:
```bash
cargo run -- 127.0.0.1 --start 1024 --end 80
```

Output:
```
Start port must be <= end port.
```

### Invalid port number:
```bash
cargo run -- 127.0.0.1 --start abc --end 443
```

Output:
```
Invalid start port: abc
```

### Missing target:
```bash
cargo run --
```

Output:
```
Usage: portscan <TARGET> [OPTIONS]
```

### Script:
"The program validates all inputs and provides helpful error messages. Let me show you some error cases..."

---

## DEMO SCENARIO 7: Performance Comparison

### Scenario A: Slow Settings
```bash
time cargo run --release -- 127.0.0.1 --start 1 --end 1000 --workers 10 --timeout-ms 500
```

### Scenario B: Fast Settings
```bash
time cargo run --release -- 127.0.0.1 --start 1 --end 1000 --workers 100 --timeout-ms 300
```

### Script:
"Notice the huge difference in execution time. The first scan with 10 workers takes maybe 30 seconds. The second scan with 100 workers finishes in about 5 seconds. That's why we use threading!"

---

## DEMO SCENARIO 8: Code Review Walkthrough

### Show the file structure:
```bash
cd a:\git\rust_portscan
tree portscan /A
```

Or PowerShell equivalent:
```powershell
Get-ChildItem portscan -Recurse | Where-Object {$_.Name -ne 'target'} | Select-Object FullName
```

### Show key parts of main.rs:

**Line 119-126 (The scanning function):**
```rust
fn scan_port(target: &str, port: u16, timeout: Duration) -> bool {
    let addr = format!("{}:{}", target, port);
    match addr.parse() {
        Ok(socket_addr) => {
            TcpStream::connect_timeout(&socket_addr, timeout).is_ok()
        }
        Err(_) => false,
    }
}
```
"This is where the actual port scanning happens - we try to make a TCP connection."

**Line 141 (Thread-safe storage):**
```rust
let open_ports: Arc<Mutex<Vec<u16>>> = Arc::new(Mutex::new(Vec::new()));
```
"This stores our results in a thread-safe way using Arc<Mutex<>>"

**Lines 154-176 (Thread spawning):**
```rust
let handle = thread::spawn(move || {
    for port in worker_start..=worker_end {
        if scan_port(&target, port, timeout) {
            println!("[+] Port {} open", port);
            open_ports.lock().unwrap().push(port);
        }
    }
});
```
"Each worker thread scans its assigned ports and reports open ones."

---

## VIDEO STRUCTURE SUGGESTION

### Opening (0:00-1:00)
1. Show the project on GitHub
2. Show what the program does: scan ports
3. Show a quick demo running

### Explanation (1:00-6:00)
1. What are ports? (1:00-1:30)
2. Why scan them? (1:30-2:00)
3. How does threading work? (2:00-3:00)
4. What is thread safety? (3:00-4:00)
5. Project overview (4:00-5:00)
6. Installation & build (5:00-6:00)

### Demos (6:00-10:00)
1. Local machine scan (6:00-7:00)
2. Narrow port range (7:00-8:00)
3. Remote host scan (8:00-9:00)
4. Code walkthrough (9:00-10:00)

### Closing (10:00-11:00)
1. Key takeaways
2. Use cases
3. Ethical considerations
4. Call to action (GitHub, subscribe, etc.)

---

## COMMON ISSUES & SOLUTIONS

### "Linker not found" error:
**Cause:** C++ build tools not installed
**Solution:** Install Visual Studio Build Tools or MinGW

### "Connection refused" for all ports:
**Cause:** No services running on localhost
**Solution:** Start a web server: `python -m http.server 8080`
Then: `cargo run -- 127.0.0.1 --start 8080 --end 8080`

### Scan is too slow:
**Cause:** Too few workers or timeout too high
**Solution:** Increase workers, decrease timeout-ms
Example: `--workers 200 --timeout-ms 100`

### Too many false negatives (missing open ports):
**Cause:** Timeout too short or workers too many
**Solution:** Increase timeout-ms
Example: `--timeout-ms 1000`

---

## SCRIPT TIPS FOR YOUR VIDEO

### When explaining threading:
"Imagine checking 100 doors one at a time takes 5 minutes. But if you have 100 friends and each checks one door simultaneously, you're done in seconds. That's what threading does!"

### When showing error handling:
"Notice the program doesn't crash on bad input - it tells you exactly what's wrong. That's good defensive programming."

### When showing thread safety:
"If multiple threads tried to write to our results list at the same time, they might corrupt the data. The Mutex ensures only one thread writes at a time, like a bathroom lock."

### When showing performance:
"See how much faster with more workers? But we're limited by network latency - can't go faster than the target responds."

---
