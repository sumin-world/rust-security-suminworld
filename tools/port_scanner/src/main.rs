use clap::{ArgGroup, Parser};
use std::{net::ToSocketAddrs, sync::Arc, time::Duration};
use tokio::{net::TcpStream, sync::Semaphore, time::timeout};

/// High-performance port scanner built with Rust
#[derive(Parser, Debug)]
#[command(name = "port_scanner", version, author = "suminworld")]
#[command(about = "Simple async TCP port scanner")]
#[command(group(
    ArgGroup::new("port_spec")
        .args(["ports", "range"])
        .required(false)
))]
struct Cli {
    /// Target hostname or IP (e.g., 192.168.1.1 or google.com)
    target: String,

    /// Comma-separated ports (e.g., 80,443,22)
    #[arg(short = 'p', long)]
    ports: Option<String>,

    /// Range (e.g., 1-1000)
    #[arg(long)]
    range: Option<String>,

    /// Fast preset (overrides timeout/concurrency/range unless -p/--range provided)
    #[arg(long)]
    fast: bool,

    /// Connection timeout in milliseconds
    #[arg(long, default_value_t = 300)]
    timeout_ms: u64,

    /// Max concurrent connections
    #[arg(long, default_value_t = 512)]
    concurrency: usize,
}

#[tokio::main]
async fn main() {
    let mut cli = Cli::parse();

    // --fast preset
    if cli.fast && cli.ports.is_none() && cli.range.is_none() {
        cli.range = Some("1-1024".to_string());
        cli.timeout_ms = 200;
        cli.concurrency = 1024;
    }

    // port set
    let ports = if let Some(spec) = &cli.ports {
        parse_ports_list(spec)
    } else if let Some(r) = &cli.range {
        parse_range(r)
    } else {
        parse_range("1-1000")
    };

    if ports.is_empty() {
        eprintln!("No valid ports to scan (check -p/--range).");
        std::process::exit(1);
    }

    let host = cli.target.clone();
    if (host.as_str(), 0).to_socket_addrs().is_err() {
        eprintln!("Invalid target hostname/IP: {}", host);
        std::process::exit(1);
    }

    let timeout_dur = Duration::from_millis(cli.timeout_ms);
    let sem = Arc::new(Semaphore::new(cli.concurrency));

    println!(
        "Target: {host}\nPorts: {} ({} total)\nTimeout: {}ms  Concurrency: {}\nStarting scan...",
        preview_ports(&ports, 20),
        ports.len(),
        cli.timeout_ms,
        cli.concurrency
    );

    let mut tasks = Vec::with_capacity(ports.len());
    for &port in &ports {
        let sem = Arc::clone(&sem);
        let host = host.clone();
        let t = tokio::spawn(async move {
            let permit = sem.acquire_owned().await.ok()?; // hold until end of scope
            let _keep = permit;
            let addr = format!("{}:{}", host, port);
            let res = timeout(timeout_dur, TcpStream::connect(&addr)).await;
            match res {
                Ok(Ok(_)) => Some(port),
                _ => None,
            }
        });
        tasks.push(t);
    }

    let mut open = Vec::new();
    for t in tasks {
        if let Ok(Some(p)) = t.await {
            println!("OPEN  {p}");
            open.push(p);
        }
    }

    println!(
        "\nDone. Open ports: {}",
        if open.is_empty() {
            "none".to_string()
        } else {
            format!("{:?}", open)
        }
    );
}

fn parse_ports_list(spec: &str) -> Vec<u16> {
    let mut out = Vec::new();
    for part in spec.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        if let Some((a, b)) = part.split_once('-') {
            out.extend(parse_range(&format!("{}-{}", a, b)));
        } else if let Ok(p) = part.parse::<u16>() {
            if p > 0 {
                out.push(p);
            }
        }
    }
    dedup_sort(out)
}

fn parse_range(spec: &str) -> Vec<u16> {
    let (a, b) = match spec.split_once('-') {
        Some((a, b)) => (a.trim(), b.trim()),
        None => return Vec::new(),
    };
    let (Ok(mut start), Ok(mut end)) = (a.parse::<u16>(), b.parse::<u16>()) else {
        return Vec::new();
    };
    if start == 0 {
        start = 1;
    }
    if end == 0 {
        end = 1;
    }
    if start > end {
        std::mem::swap(&mut start, &mut end);
    }
    (start..=end).collect()
}

fn dedup_sort(mut v: Vec<u16>) -> Vec<u16> {
    v.sort_unstable();
    v.dedup();
    v
}

fn preview_ports(ports: &Vec<u16>, max_show: usize) -> String {
    if ports.len() <= max_show {
        return format!("{:?}", ports);
    }
    let mut first = ports[..max_show].to_vec();
    let last = *ports.last().unwrap();
    first.push(last);
    format!("{:?} … (truncated)", first)
}
