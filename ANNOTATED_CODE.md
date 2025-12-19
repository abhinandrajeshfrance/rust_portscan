# Annotated Source Code for Video

This file shows the exact code with video commentary annotations.

---

## PART 1: Imports & Configuration

```rust
// ===== VIDEO CAPTION: "Importing Rust Standard Library" =====
use std::env;              // Read command-line arguments
use std::net::TcpStream;   // TCP network connections (THE KEY!)
use std::sync::{Arc, Mutex}; // Thread-safe shared data
use std::thread;           // Spawn worker threads
use std::time::Duration;   // Set timeouts

// KEY POINT FOR VIDEO: No external crates! Everything built into Rust.
// This is a ~200 line pure Rust program.

// ===== VIDEO CAPTION: "Configuration Structure" =====
struct Args {
    target: String,      // What IP/hostname to scan
    start: u16,          // First port (1-65535)
    end: u16,            // Last port (1-65535)
    workers: usize,      // How many threads to spawn
    timeout_ms: u64,     // Connection timeout in milliseconds
}

impl Default for Args {
    fn default() -> Self {
        Args {
            target: String::new(),
            start: 1,           // Default: scan common ports
            end: 1024,          // Default: up to port 1024
            workers: 100,       // Default: 100 parallel threads
            timeout_ms: 300,    // Default: wait 300ms per port
        }
    }
}

// EXPLAIN IN VIDEO:
// "These defaults mean users don't need to specify everything.
// cargo run -- 127.0.0.1 automatically uses these values."
```

---

## PART 2: Argument Parsing

```rust
// ===== VIDEO CAPTION: "Parsing Command Line Arguments" =====
fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();
    //                      ^^^^^^^^^^^ Get all command-line inputs
    // If user types: cargo run -- 127.0.0.1 --start 1 --end 1024
    // Then args = ["portscan", "127.0.0.1", "--start", "1", "--end", "1024"]

    // Need at least: program name + target
    if args.len() < 2 {
        return Err("Usage: portscan <TARGET> [OPTIONS]...".to_string());
    }

    let mut parsed = Args::default();  // Start with defaults
    parsed.target = args[1].clone();   // First argument is the target
    // parsed.target = "127.0.0.1"

    // Now parse optional flags
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            // ===== Handle --start flag =====
            "--start" | "-s" => {
                if i + 1 < args.len() {
                    // Try to parse next argument as a number
                    parsed.start = args[i + 1]
                        .parse()  // Convert "1024" to u16: 1024
                        .map_err(|_| format!("Invalid start port: {}", args[i + 1]))?;
                    i += 2;  // Skip past the flag and its value
                } else {
                    return Err("--start requires a value".to_string());
                }
            }
            
            // ===== Handle --end flag =====
            "--end" | "-e" => {
                if i + 1 < args.len() {
                    parsed.end = args[i + 1]
                        .parse()
                        .map_err(|_| format!("Invalid end port: {}", args[i + 1]))?;
                    i += 2;
                } else {
                    return Err("--end requires a value".to_string());
                }
            }
            
            // ===== Handle --workers flag =====
            "--workers" | "-w" => {
                if i + 1 < args.len() {
                    parsed.workers = args[i + 1]
                        .parse()
                        .map_err(|_| format!("Invalid workers count: {}", args[i + 1]))?;
                    i += 2;
                } else {
                    return Err("--workers requires a value".to_string());
                }
            }
            
            // ===== Handle --timeout-ms flag =====
            "--timeout-ms" => {
                if i + 1 < args.len() {
                    parsed.timeout_ms = args[i + 1]
                        .parse()
                        .map_err(|_| format!("Invalid timeout: {}", args[i + 1]))?;
                    i += 2;
                } else {
                    return Err("--timeout-ms requires a value".to_string());
                }
            }
            
            // ===== Handle --help flag =====
            "--help" | "-h" => {
                return Err("Help: ...\n...".to_string());
            }
            
            // ===== Unknown flag =====
            _ => {
                return Err(format!("Unknown option: {}", args[i]));
            }
        }
    }

    Ok(parsed)  // Return successfully parsed arguments
}

// EXPLAIN IN VIDEO:
// "This function validates all input and returns either:
// - Ok(Args) with valid configuration
// - Err(String) with error message
// This prevents the program from running with invalid parameters!"
```

---

## PART 3: The Core Scanning Function

```rust
// ===== VIDEO CAPTION: "The Heart of the Scanner" =====
// THIS IS WHERE THE ACTUAL PORT SCANNING HAPPENS

fn scan_port(target: &str, port: u16, timeout: Duration) -> bool {
    //          ^^^^^^^^^  ^^^^^^^  ^^^^^^^^^^^^^^^^^
    //          Target IP  Port #   Connection timeout
    
    // Example: scan_port("127.0.0.1", 80, 300ms)
    
    // Step 1: Create address string
    let addr = format!("{}:{}", target, port);
    // Result: "127.0.0.1:80"
    
    // Step 2: Parse string into socket address
    match addr.parse() {
        Ok(socket_addr) => {
            // "127.0.0.1:80" is a valid address ✓
            
            // Step 3: ATTEMPT TCP CONNECTION
            TcpStream::connect_timeout(&socket_addr, timeout).is_ok()
            //        ^^^^^^^^^^^^^^^^^
            //        Try to connect. Wait up to 300ms for response.
            //
            // If service is listening on port 80:
            //   Connection succeeds → .is_ok() returns TRUE
            //   "Port 80 is OPEN!" ✓
            //
            // If nothing listening on port 80:
            //   Connection refused → .is_ok() returns FALSE  
            //   "Port 80 is CLOSED!"
            //
            // If network times out:
            //   Wait 300ms then give up → .is_ok() returns FALSE
            //   "Port 80 is CLOSED/FILTERED!"
        }
        Err(_) => {
            // "127.0.0.1:80" is invalid format → Port closed
            false
        }
    }
}

// EXPLAIN IN VIDEO (VERY CAREFULLY):
// "This function is simple but powerful. It tries to make a TCP connection.
// TCP is like a phone call - the port either answers (open) or doesn't (closed).
// We wait 300ms for the target to respond. If no response, the port is closed."

// Show on screen:
// "Sending TCP SYN packet to 127.0.0.1:80"
// "Target responds with SYN-ACK → Connection succeeds → Port OPEN"
// vs.
// "Sending TCP SYN packet to 127.0.0.1:9999"  
// "No response after 300ms → Port CLOSED/FILTERED"
```

---

## PART 4: Main Function

```rust
fn main() {
    // ===== VIDEO CAPTION: "Step 1: Parse and Validate Arguments" =====
    let args = match parse_args() {
        Ok(a) => a,  // Got valid arguments
        Err(e) => {
            eprintln!("{}", e);  // Print error message
            std::process::exit(1);  // Exit with failure code
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

    // ===== VIDEO CAPTION: "Print Welcome Banner" =====
    println!("[*] Scanning {} from ports {} to {} with {} workers...",
        args.target, args.start, args.end, args.workers);
    println!("[*] Timeout: {} ms\n", args.timeout_ms);
    println!("[!] Only scan systems you own or have explicit permission to test.\n");
    //     ^^^ ETHICAL WARNING

    // ===== VIDEO CAPTION: "Step 2: Create Thread-Safe Results Storage" =====
    let open_ports: Arc<Mutex<Vec<u16>>> = Arc::new(Mutex::new(Vec::new()));
    //  ^^^^^^^^^^ List of open ports
    //            ^^^^^^^^^^^^^ Can only be accessed one thread at a time
    //                         ^^^^^^^^ Multiple threads can share this
    
    // WHITEBOARD EXPLANATION:
    // Arc<Mutex<Vec<u16>>>
    //     │      │      │
    //     │      │      └─ Vector of 16-bit unsigned integers (port numbers)
    //     │      └─ Mutex lock (only one thread accesses at a time)
    //     └─ Arc (multiple threads can own references to this)

    // ===== VIDEO CAPTION: "Step 3: Calculate Work Distribution" =====
    let total_ports = (args.end - args.start + 1) as usize;
    // Example: ports 1-1024 → total = 1024 ports
    
    let num_workers = args.workers.min(total_ports.max(1));
    // Don't create more workers than ports
    // Example: 1024 ports with 100 requested workers → use 100 workers
    
    let mut handles = Vec::new();
    // Will store handles to all worker threads
    
    let ports_per_worker = (total_ports + num_workers - 1) / num_workers;
    // Divide evenly. Example: 1024 ports / 100 workers = 10.24 → 11 per worker
    
    // DRAW ON SCREEN:
    // 1024 ports to scan
    // ├─ Worker 1: ports 1-11
    // ├─ Worker 2: ports 12-22
    // ├─ Worker 3: ports 23-33
    // ...
    // └─ Worker 100: ports 1014-1024
    //
    // All start at same time → Parallel processing!

    // ===== VIDEO CAPTION: "Step 4: Spawn Worker Threads" =====
    for i in 0..num_workers {
        // Calculate port range for this worker
        let start_index = i * ports_per_worker;
        let end_index = ((i + 1) * ports_per_worker).min(total_ports);

        if start_index >= end_index {
            continue;  // Skip if no ports assigned
        }

        let worker_start = args.start + start_index as u16;
        let worker_end = args.start + (end_index as u16) - 1;

        // Clone data for this thread
        let target = args.target.clone();           // Copy string
        let timeout = Duration::from_millis(args.timeout_ms);
        let open_ports = Arc::clone(&open_ports);   // Share ownership
        //              ^^^^ Only increments reference counter!
        //              Doesn't copy the entire vector!

        // ===== SPAWN THE ACTUAL THREAD =====
        let handle = thread::spawn(move || {
            //                      ^^^^ 'move' captures cloned variables
            
            // This code runs on a separate thread!
            for port in worker_start..=worker_end {
                // Scan each port assigned to this worker
                if scan_port(&target, port, timeout) {
                    // PORT IS OPEN!
                    println!("[+] Port {} open", port);
                    
                    // Record this open port (thread-safe!)
                    open_ports.lock().unwrap().push(port);
                    //         ^^^^ ACQUIRE LOCK (wait for other threads)
                    //              .unwrap() → Get the vector
                    //                          .push() → Add port number
                    //         Then automatically RELEASE LOCK
                }
            }
            // When this thread finishes its loop, it exits
        });

        handles.push(handle);  // Save handle to wait later
    }
    
    // At this point:
    // - Main thread is running
    // - 100 worker threads are all scanning SIMULTANEOUSLY
    // - Data in open_ports is thread-safe

    // ===== VIDEO CAPTION: "Step 5: Wait for All Threads to Finish" =====
    // Wait for all threads to complete
    for h in handles {
        if let Err(e) = h.join() {
            //              ^^^^ Block until thread finishes
            eprintln!("Worker thread panicked: {:?}", e);
        }
    }
    
    // SHOW TIMELINE:
    // T=0ms:    All 100 threads spawn and start scanning
    // T=100ms:  Thread 5 finds port 80 open, prints "[+] Port 80 open"
    // T=200ms:  Thread 12 finds port 443 open
    // T=500ms:  First few threads finish their ports
    // T=2000ms: Last thread finishes
    //           All threads now complete
    //           Main thread wakes up and continues

    // ===== VIDEO CAPTION: "Step 6: Display Results" =====
    let mut open = open_ports.lock().unwrap();
    // ^^^^^^^^^^^ Lock and get vector (main thread, no others running)
    
    open.sort_unstable();
    // Sort port numbers in ascending order
    // [443, 22, 80, 3306] → [22, 80, 443, 3306]

    println!("\n[*] Scan complete.");
    if open.is_empty() {
        println!("[-] No open ports found in the specified range.");
    } else {
        println!("[+] Open ports found:");
        for p in open.iter() {
            println!("    - {}", p);
        }
    }
    // Program exits here
}

// FINAL EXPLAIN IN VIDEO:
// "Notice the entire scan happened without any race conditions,
// data corruption, or crashes. That's Rust's strength:
// Safe multithreading by default!"
```

---

## Key Points to Highlight in Video

1. **No External Dependencies**
   - Point out: All imports are `std::*`
   - This means no dependency hell, no vulnerability updates needed

2. **Thread Safety by Design**
   - Arc<Mutex<>> prevents data corruption
   - Compiler enforces this at compile time
   - Can't accidentally create a data race

3. **Error Handling**
   - `.parse()` returns Result<T, E>
   - `.map_err()` chains error handling
   - `.join()` waits for threads and handles panics

4. **Performance**
   - 1024 ports with 100 workers
   - Each worker gets ~10 ports
   - Scanned in parallel, not sequentially

5. **Practical Networking**
   - `TcpStream::connect_timeout()` is the core logic
   - Shows connection attempt, not port knock
   - Only detects listening services

---

## Video Production Notes

### Show These Code Sections in This Order:

1. **Imports (5 seconds)**
   - Highlight: "No external crates"

2. **scan_port() function (30 seconds)**
   - This is THE core logic
   - Pause on `TcpStream::connect_timeout`
   - Explain TCP connection

3. **Thread spawning loop (1 minute)**
   - Show Arc::clone() and Arc::clone(&open_ports)
   - Show thread::spawn(move || { ... })
   - Show open_ports.lock().unwrap().push(port)

4. **Main thread waiting (30 seconds)**
   - Show h.join() blocking

5. **Results display (20 seconds)**
   - Show open.sort_unstable()
   - Show the println! loop

---

## Comments for Different Experience Levels

### For Beginners:
"See how we use `match`? That's Rust's way of handling both success and error cases. This prevents 'null pointer crashes' that happen in other languages."

### For Intermediate:
"Notice the `'move` keyword in `thread::spawn(move || ...)`? That transfers ownership of the cloned variables into the closure, allowing the thread to own them."

### For Advanced:
"The `Arc<Mutex<T>>` is the canonical way to share mutable state across threads. Could alternatively use channels for message passing, but Mutex is simpler here."

---
