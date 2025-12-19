# Complete Code Walkthrough for Your Video

## File: src/main.rs (202 lines total)

---

## SECTION 1: IMPORTS (Lines 1-5)

```rust
use std::env;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
```

**What each import does:**
- `std::env` - Access command-line arguments
- `std::net::TcpStream` - Create TCP connections for port scanning
- `std::sync::Arc, Mutex` - Thread-safe shared data
- `std::thread` - Spawn worker threads
- `std::time::Duration` - Set connection timeouts

**Key point:** No external crates! Everything is built into Rust's standard library.

---

## SECTION 2: CONFIGURATION STRUCT (Lines 7-22)

```rust
struct Args {
    target: String,
    start: u16,
    end: u16,
    workers: usize,
    timeout_ms: u64,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            target: String::new(),
            start: 1,
            end: 1024,
            workers: 100,
            timeout_ms: 300,
        }
    }
}
```

**What it does:**
- `Args` struct holds the scanner configuration
- `target`: IP address or hostname to scan
- `start` / `end`: Port range (1-65535 valid)
- `workers`: Number of parallel threads (default: 100)
- `timeout_ms`: How long to wait for each port (default: 300ms)

**Default impl:** Provides sensible defaults so users don't need to specify everything.

---

## SECTION 3: ARGUMENT PARSING (Lines 24-107)

```rust
fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: portscan <TARGET> ...".to_string());
    }

    let mut parsed = Args::default();
    parsed.target = args[1].clone();

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--start" | "-s" => {
                if i + 1 < args.len() {
                    parsed.start = args[i + 1]
                        .parse()
                        .map_err(|_| format!("Invalid start port: {}", args[i + 1]))?;
                    i += 2;
                } else {
                    return Err("--start requires a value".to_string());
                }
            }
            // Similar for --end, --workers, --timeout-ms
            "--help" | "-h" => {
                return Err("Help message...".to_string());
            }
            _ => {
                return Err(format!("Unknown option: {}", args[i]));
            }
        }
    }

    Ok(parsed)
}
```

**How it works:**

1. **Collect arguments**: `env::args()` gets all command-line inputs
   ```
   Example: ["portscan", "127.0.0.1", "--start", "1", "--end", "1024"]
   ```

2. **Validate minimum args**: Need at least program name + target
   ```rust
   if args.len() < 2 {
       return Err("Need a target!");
   }
   ```

3. **Parse target**: The first argument after program name
   ```rust
   parsed.target = args[1].clone();
   ```

4. **Parse options in a loop**: Match flags and read their values
   ```rust
   "--start" => parsed.start = parse_next_value()
   "--end" => parsed.end = parse_next_value()
   ```

5. **Error handling**: If parsing fails, show why:
   ```
   "Invalid start port: abc" (if user typed non-number)
   "--start requires a value" (if no value provided)
   ```

**Result:** Fully validated configuration ready for scanning

---

## SECTION 4: THE CORE SCANNING FUNCTION (Lines 119-126)

### THIS IS THE HEART OF THE PROJECT

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

**Step by step:**

1. **Create address string**
   ```rust
   let addr = format!("{}:{}", target, port);
   // Result: "127.0.0.1:80" or "192.168.1.1:443"
   ```

2. **Parse into socket address**
   ```rust
   match addr.parse() {
       Ok(socket_addr) => { ... }    // "127.0.0.1:80" → valid address
       Err(_) => false,              // Invalid format → port closed
   }
   ```

3. **Attempt TCP connection**
   ```rust
   TcpStream::connect_timeout(&socket_addr, timeout).is_ok()
   ```
   - Tries to connect to that address and port
   - Waits up to `timeout` milliseconds
   - Returns `Ok()` if connection succeeds (port OPEN)
   - Returns `Err()` if connection fails or times out (port CLOSED)
   - `.is_ok()` converts to boolean: true for open, false for closed

**Example execution:**
```
scan_port("127.0.0.1", 80, 300ms)
  → addr = "127.0.0.1:80"
  → Try TCP connection to 127.0.0.1:80
  → If Apache/nginx is listening: Connection OK → return true
  → If nothing listening: Connection refused → return false
```

---

## SECTION 5: MAIN FUNCTION START (Lines 129-168)

### Part A: Setup

```rust
fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Validate port range
    if args.start == 0 || args.end == 0 {
        eprintln!("Ports must be between 1 and 65535.");
        std::process::exit(1);
    }
    if args.start > args.end {
        eprintln!("Start port must be <= end port.");
        std::process::exit(1);
    }

    println!("[*] Scanning {} from ports {} to {} with {} workers...",
        args.target, args.start, args.end, args.workers);
    println!("[*] Timeout: {} ms\n", args.timeout_ms);
    println!("[!] Only scan systems you own or have explicit permission to test.\n");
```

**What happens:**

1. **Parse arguments** with error handling
   ```rust
   match parse_args() {
       Ok(a) => use the arguments,
       Err(e) => {
           print error message
           exit program with code 1 (failure)
       }
   }
   ```

2. **Validate port range**
   - Ports must be 1-65535 (0 is invalid, 65536+ doesn't exist)
   - Start must be ≤ End

3. **Print banner**
   - Shows what scan is being performed
   - Ethical warning

---

### Part B: Thread-Safe Results Storage (Line 141)

```rust
let open_ports: Arc<Mutex<Vec<u16>>> = Arc::new(Mutex::new(Vec::new()));
```

**This is CRUCIAL for thread safety!**

Breaking it down:

1. **`Vec<u16>`** = A growable vector (list) of 16-bit unsigned integers
   - Stores port numbers: `[22, 80, 443, ...]`
   - Starts empty

2. **`Mutex<Vec<u16>>`** = Wrap the vector in a mutex lock
   - `Mutex` = "Mutual Exclusion"
   - Only ONE thread can access/modify the vector at a time
   - Other threads must wait their turn
   - Prevents data corruption from concurrent writes

3. **`Arc<...>`** = "Atomic Reference Counted"
   - Multiple threads can own references to the same data
   - `Arc` keeps track of how many owners exist
   - When last owner is dropped, data is freed
   - Enables sharing between threads

**Visualization:**
```
Main thread creates:
    Arc<Mutex<Vec>> = Arc(Ref count: 1)

Thread 1 starts:
    Increment Arc ref count: 2
    Try to lock Mutex: LOCKED ✓ (writes port 22)
    Release Mutex lock

Thread 2 starts:
    Increment Arc ref count: 3
    Try to lock Mutex: LOCKED ✓ (writes port 80)
    Release Mutex lock

Thread 3 starts:
    Increment Arc ref count: 4
    Try to lock Mutex: LOCKED ✓ (writes port 443)
    Release Mutex lock

All threads finish:
    Arc ref count: 1
    Mutex released
    Vector safely contains: [22, 80, 443]
```

---

### Part C: Calculate Work Distribution (Line 143-152)

```rust
let total_ports = (args.end - args.start + 1) as usize;
let num_workers = args.workers.min(total_ports.max(1));

let mut handles = Vec::new();
let ports_per_worker = (total_ports + num_workers - 1) / num_workers;
```

**Example: Scanning ports 1-1024 with 100 workers**

1. **Calculate total ports**
   ```
   total_ports = (1024 - 1 + 1) = 1024
   ```

2. **Limit workers to not exceed total ports**
   ```
   num_workers = min(100, 1024) = 100
   ```

3. **Divide evenly**
   ```
   ports_per_worker = (1024 + 100 - 1) / 100 = 1123 / 100 = 11.23 → 11 ports each
   ```

   So:
   - Workers 1-93: Each gets 11 ports
   - Workers 94-100: Get remaining ports
   - Total distributed: 1024 ports

---

### Part D: Thread Spawning (Lines 154-176)

```rust
for i in 0..num_workers {
    let start_index = i * ports_per_worker;
    let end_index = ((i + 1) * ports_per_worker).min(total_ports);

    if start_index >= end_index {
        continue;  // Skip if no ports assigned
    }

    let worker_start = args.start + start_index as u16;
    let worker_end = args.start + (end_index as u16) - 1;

    let target = args.target.clone();
    let timeout = timeout;
    let open_ports = Arc::clone(&open_ports);

    let handle = thread::spawn(move || {
        for port in worker_start..=worker_end {
            if scan_port(&target, port, timeout) {
                println!("[+] Port {} open", port);
                open_ports.lock().unwrap().push(port);
            }
        }
    });

    handles.push(handle);
}
```

**What this does:**

1. **Loop through each worker**
   ```rust
   for i in 0..num_workers {  // i goes 0, 1, 2, ..., 99
   ```

2. **Calculate port range for this worker**
   ```rust
   start_index = 0 * 11 = 0        // Worker 0
   end_index = min(1 * 11, 1024) = 11

   Worker 0 gets indices 0-10, which are ports 1-11
   
   start_index = 1 * 11 = 11       // Worker 1
   end_index = min(2 * 11, 1024) = 22

   Worker 1 gets indices 11-21, which are ports 12-22
   ```

3. **Clone data for the thread**
   ```rust
   let target = args.target.clone();        // Clone target string
   let open_ports = Arc::clone(&open_ports); // Clone Arc (not data)
   ```
   Note: `Arc::clone()` just increments reference count, doesn't clone the vector!

4. **Spawn the actual worker thread**
   ```rust
   let handle = thread::spawn(move || {  // 'move' captures cloned variables
       for port in worker_start..=worker_end {
           if scan_port(&target, port, timeout) {
               println!("[+] Port {} open", port);
               
               // THREAD-SAFE WRITE
               open_ports.lock().unwrap().push(port);
               //         ^^^^^ Acquire lock
               //                            ^^^^^ Release lock (unwrap)
           }
       }
   });
   ```

5. **Save thread handle for later**
   ```rust
   handles.push(handle);  // We'll wait for these to finish
   ```

---

### Part E: Wait for All Threads (Lines 183-188)

```rust
// Wait for all threads to complete
for h in handles {
    if let Err(e) = h.join() {
        eprintln!("Worker thread panicked: {:?}", e);
    }
}
```

**What happens:**

1. **`.join()` on each handle** - Waits for thread to finish
   ```
   Main thread blocks here until:
   - Thread 1 scans all its ports
   - Thread 2 scans all its ports
   - ... and so on for all 100 threads
   ```

2. **Error handling** - If a thread crashes
   ```rust
   if let Err(e) = h.join() {
       // Thread panicked, print error
   }
   ```

**Timeline:**
```
T=0ms    All 100 threads start scanning simultaneously

T=100ms  Threads 5, 12, 47 find open ports, print results

T=200ms  Threads 3, 89, 99 find more ports

T=500ms  Thread 42 finished scanning its ports
         Thread 55 finished scanning its ports

T=1000ms All remaining threads finish

        Main thread wakes up, continues to results
```

---

### Part F: Display Results (Lines 190-202)

```rust
let mut open = open_ports.lock().unwrap();
open.sort_unstable();

println!("\n[*] Scan complete.");
if open.is_empty() {
    println!("[-] No open ports found in the specified range.");
} else {
    println!("[+] Open ports found:");
    for p in open.iter() {
        println!("    - {}", p);
    }
}
```

**Final step:**

1. **Acquire final lock and get results**
   ```rust
   let mut open = open_ports.lock().unwrap();
   //              ^^^^ Lock the mutex
   //                             ^^^^ Get the vector
   ```

2. **Sort port numbers**
   ```rust
   open.sort_unstable();  // [443, 22, 80] → [22, 80, 443]
   ```

3. **Display results**
   ```
   If empty:
   [-] No open ports found in the specified range.
   
   If found:
   [+] Open ports found:
       - 22
       - 80
       - 443
   ```

---

## COMPLETE EXECUTION FLOW

Here's what happens from start to finish:

```
User runs: cargo run -- 127.0.0.1 --start 1 --end 1024 --workers 100

1. main() called
2. parse_args() extracts configuration
3. Validation: ensure ports 1-1024 are valid ✓
4. Create Arc<Mutex<Vec>> for results
5. Spawn 100 worker threads
   - Each thread gets a range of ports
   - Each thread starts scanning IMMEDIATELY

6. Main thread waits at .join() calls
   
7. Worker threads (concurrent):
   Thread 1: Tries port 1... port 2... port 3... ...port 10
   Thread 2: Tries port 11... port 12... ...port 20
   ...all scanning at the same time...
   
   When port 22 is found open:
   Thread 1 prints: "[+] Port 22 open"
   Thread 1 locks mutex and adds 22 to vector
   
   When port 80 is found open:
   Thread 2 prints: "[+] Port 80 open"
   Thread 2 locks mutex and adds 80 to vector

8. All workers finish (total time ~5-10 seconds)

9. Main thread wakes up
10. Lock mutex and get the vector: [22, 80, 443]
11. Sort: [22, 80, 443]
12. Print results
13. Program exits
```

---

## KEY CONCEPTS FOR YOUR VIDEO

### 1. **TCP Connection**
"A TCP connection is like a phone call - you dial the number (IP:port) and wait for someone to answer. If they answer, the port is OPEN. If nobody answers, the port is CLOSED."

### 2. **Threading**
"Instead of dialing ports one at a time, we dial 100 at once. One person handles 10 ports, another handles the next 10, etc. All simultaneously."

### 3. **Thread Safety**
"Multiple people can't write to the same list at the same time, or they'll corrupt the data. A Mutex is like a checkpoint - only one person can write at a time."

### 4. **Arc (Atomic Reference Counting)**
"Like a library book with multiple due dates - each thread 'checks out' a reference. When the last person returns it, the book goes away."

### 5. **Performance**
"Without threading: 1024 ports × 300ms = 307 seconds
With 100 threads: ~1024/100 × 300ms = 3 seconds
THAT'S 100X FASTER!"

---

## Code Quality Notes

### ✓ What's Good
- No external dependencies (pure std lib only)
- Thread-safe by design
- Proper error handling
- Input validation
- Clean code structure

### ⚠️ Minor Issues (Clippy warnings)
1. Line 39: Could initialize Args with struct literal
2. Line 152: Could use `.div_ceil()` instead of manual calculation
3. Line 166: Redundant `timeout` variable

These are style issues, not functionality problems.

---
