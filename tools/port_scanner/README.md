# Port Scanner

High-performance async TCP port scanner built with Rust.

## Features

- **Async/Concurrent Scanning** - Fast parallel port scanning using Tokio
- **Flexible Port Specification** - Support for ranges, lists, and combinations
- **Configurable Timeouts** - Adjustable connection timeout settings
- **Concurrency Control** - Limit concurrent connections to avoid overwhelming targets
- **Fast Preset** - Quick scan mode for common ports with optimized settings

## Usage

### Basic Scanning

```bash
# Quick scan of common ports (1-1024)
cargo run -p port_scanner -- target.com --fast

# Scan specific ports
cargo run -p port_scanner -- 192.168.1.1 -p 22,80,443,8080

# Scan port range
cargo run -p port_scanner -- scanme.nmap.org --range 1-1000
```

### Advanced Options

```bash
# Custom timeout and concurrency
cargo run -p port_scanner -- target.com --range 1-65535 --timeout-ms 500 --concurrency 200

# Mix of ranges and specific ports
cargo run -p port_scanner -- target.com -p 21,22,80-90,443,8000-8080
```

### Command Line Options

```
Usage: port_scanner [OPTIONS] <TARGET>

Arguments:
  <TARGET>  Target hostname or IP (e.g., 192.168.1.1 or google.com)

Options:
  -p, --ports <PORTS>              Comma-separated ports (e.g., 80,443,22)
      --range <RANGE>              Range (e.g., 1-1000)
      --fast                       Fast preset (overrides timeout/concurrency/range unless -p/--range provided)
      --timeout-ms <TIMEOUT_MS>    Connection timeout in milliseconds [default: 300]
      --concurrency <CONCURRENCY>  Max concurrent connections [default: 512]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Examples

### Quick Network Discovery
```bash
# Fast scan of local network gateway
cargo run -p port_scanner -- 192.168.1.1 --fast

# Scan web services
cargo run -p port_scanner -- example.com -p 80,443,8080,8443
```

### Security Testing
```bash
# Full port scan (be careful with external targets)
cargo run -p port_scanner -- target.local --range 1-65535 --timeout-ms 1000

# Common service ports
cargo run -p port_scanner -- target.local -p 21,22,23,25,53,80,110,443,993,995
```

## Technical Details

- **Async Runtime**: Built on Tokio for high-performance concurrent I/O
- **Connection Handling**: TCP connection attempts with configurable timeouts  
- **Memory Efficient**: Minimal memory footprint with streaming results
- **Error Handling**: Graceful handling of connection failures and timeouts

## Performance

The scanner uses semaphore-based concurrency control to balance speed with resource usage:

- **Default Settings**: 512 concurrent connections, 300ms timeout
- **Fast Mode**: 1024 concurrent connections, 200ms timeout, ports 1-1024
- **Custom Tuning**: Adjust based on target network and local resources

## Ethical Use

This tool is designed for:
- Network administration and troubleshooting
- Security testing of systems you own or have permission to test
- Educational purposes and learning network concepts

Always ensure you have proper authorization before scanning any systems.
