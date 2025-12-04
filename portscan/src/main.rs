use std::env;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(
            "Usage: portscan <TARGET> [--start N] [--end M] [--workers W] [--timeout-ms T]\n\
             Example: portscan 127.0.0.1 --start 1 --end 1024 --workers 100 --timeout-ms 300"
                .to_string(),
        );
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
            "--help" | "-h" => {
                return Err(
                    "Simple TCP Port Scanner\n\n\
                     Usage: portscan <TARGET> [OPTIONS]\n\n\
                     Arguments:\n\
                       <TARGET>  Target IPv4/hostname (e.g., 127.0.0.1 or scanme.nmap.org)\n\n\
                     Options:\n\
                       -s, --start <START>       Start of TCP port range (default: 1)\n\
                       -e, --end <END>           End of TCP port range (default: 1024)\n\
                       -w, --workers <WORKERS>   Max number of worker threads (default: 100)\n\
                       --timeout-ms <MS>        Connection timeout in milliseconds (default: 300)\n\
                       -h, --help                Print this help message"
                        .to_string(),
                );
            }
            _ => {
                return Err(format!("Unknown option: {}", args[i]));
            }
        }
    }

    Ok(parsed)
}

fn scan_port(target: &str, port: u16, timeout: Duration) -> bool {
    let addr = format!("{}:{}", target, port);
    match addr.parse() {
        Ok(socket_addr) => {
            TcpStream::connect_timeout(&socket_addr, timeout).is_ok()
        }
        Err(_) => false,
    }
}

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

    println!(
        "[*] Scanning {} from ports {} to {} with {} workers...",
        args.target, args.start, args.end, args.workers
    );
    println!("[*] Timeout: {} ms\n", args.timeout_ms);
    println!("[!] Only scan systems you own or have explicit permission to test.\n");

    let timeout = Duration::from_millis(args.timeout_ms);
    let open_ports: Arc<Mutex<Vec<u16>>> = Arc::new(Mutex::new(Vec::new()));

    let total_ports = (args.end - args.start + 1) as usize;
    let num_workers = args.workers.min(total_ports.max(1));

    // Split the port range across workers
    let mut handles = Vec::new();
    let ports_per_worker = (total_ports + num_workers - 1) / num_workers;

    for i in 0..num_workers {
        let start_index = i * ports_per_worker;
        let end_index = ((i + 1) * ports_per_worker).min(total_ports);

        if start_index >= end_index {
            continue;
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

    // Wait for all threads to complete
    for h in handles {
        if let Err(e) = h.join() {
            eprintln!("Worker thread panicked: {:?}", e);
        }
    }

    // Sort and display results
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
}
