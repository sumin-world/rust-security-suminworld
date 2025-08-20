# sum-rust-secu-lab

> Security tools and research built with Rust

## About

Personal security laboratory for learning cybersecurity concepts through hands-on Rust development. Starting with basic network tools and gradually expanding into various security domains.

## Repository Structure

```
sum-rust-secu-lab/
├── src/
│   └── main.rs              # Port scanner (current)
├── tools/                   # Individual security tools
│   ├── hash_cracker/        # Password and hash analysis
│   ├── log_analyzer/        # Security log analysis
│   ├── web_fuzzer/          # Directory and parameter fuzzing
│   └── packet_sniffer/      # Network traffic analysis
├── research/                # Security research and experiments
│   ├── protocols/           # Network protocol analysis
│   ├── crypto/              # Cryptographic implementations
│   └── vulns/               # Vulnerability research
├── docs/                    # Learning notes and documentation
│   ├── learning_notes.md    # Study progress and concepts
│   ├── tool_usage.md        # Usage guides and examples
│   └── references.md        # Useful resources and links
├── examples/                # Example usage and test cases
└── README.md
```

## Current Tools

### Port Scanner
High-performance async TCP port scanner with concurrent connection handling.
→ See [tools/port_scanner/](tools/port_scanner/) for detailed usage and features.

## Planned Tools

- **Hash Cracker** - Multi-algorithm password and hash analysis
- **Log Analyzer** - Security log parsing and correlation
- **Web Fuzzer** - Directory and parameter discovery
- **Packet Sniffer** - Real-time network traffic analysis
- **Crypto Tools** - Cryptographic algorithm implementations

## Learning Focus

- **Async Rust Programming** - Tokio, concurrency patterns
- **Network Security** - TCP/IP, protocols, scanning techniques  
- **Performance Optimization** - Memory safety, zero-cost abstractions
- **CLI Tool Design** - User experience, argument parsing
- **Security Concepts** - Reconnaissance, analysis, defensive tools

## Getting Started

```bash
git clone https://github.com/suminworld/sum-rust-secu-lab.git
cd sum-rust-secu-lab
cargo build --release

# Run port scanner
cargo run -- --help
```

## Disclaimer

These tools are for educational and authorized testing purposes only. Always ensure you have permission before testing any systems you don't own.
