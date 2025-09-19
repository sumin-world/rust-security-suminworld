# rust-security-suminworld 🦀🔒

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub last commit](https://img.shields.io/github/last-commit/sumin-world/rust-security-suminworld)](https://github.com/sumin-world/rust-security-suminworld/commits/main)
[![GitHub stars](https://img.shields.io/github/stars/sumin-world/rust-security-suminworld?style=social)](https://github.com/sumin-world/rust-security-suminworld/stargazers)

> **Rust-based Security Research Lab**  
> Passive Wi-Fi auditing, packet fuzzing, Discord security bots, and educational crypto & kernel exploit modules.

> A comprehensive collection of Rust-based cybersecurity tools and research projects designed for learning and demonstrating security concepts. This repository showcases Rust's capabilities in building high-performance, memory-safe security applications.

---

## 🔖 Topics / Tags
`rust` · `security` · `cybersecurity` · `wifi` · `wireless-security` ·  
`penetration-testing` · `80211` · `pcap` · `rust-security` · `passive-monitoring`

## 🎯 Overview

**rust-security-suminworld** is a comprehensive collection of Rust-based cybersecurity tools and research projects designed for learning and demonstrating security concepts. This repository showcases Rust's capabilities in building high-performance, memory-safe security applications.

### 🌟 Key Features

- **High Performance**: Leverages Rust's zero-cost abstractions and async capabilities
- **Memory Safety**: Eliminates common security vulnerabilities through Rust's ownership system  
- **Concurrent Processing**: Utilizes Tokio for high-performance async operations
- **Educational Focus**: Comprehensive documentation and learning resources
- **Modular Architecture**: Clean separation of tools, research, and documentation

## 🧰 Tools & Modules

This repository is a collection of Rust-based security research tools and educational modules.  
Each tool is self-contained with its own documentation and can be built independently.

| Module | Description | Status | Link |
|--------|-------------|--------|------|
| 🔐 **discord_audit_bot** | Security audit bot for Discord servers (Rust + Serenity) | ✅ Complete | [tools/discord_audit_bot](./tools/discord_audit_bot/) |
| 🌐 **packet-match-fuzz** | Pattern-matching fuzzer for packet payloads using KMP algorithm | ✅ Complete | [tools/packet-match-fuzz](./tools/packet-match-fuzz/) |
| 📡 **wifi_audit** | Passive Wi-Fi auditing tool (802.11 Beacon/Probe analysis, monitor mode) | ✅ Complete | [tools/wifi_audit](./tools/wifi_audit/) |
| 🔑 **crypto** | Educational implementations of classical/symmetric/asymmetric ciphers & hashing | ✅ Complete | [research/crypto](./research/crypto/) |
| 🛡️ **linux-kernel-exploits** | Educational CVE labs & write-ups for kernel exploitation research | ✅ Complete | [linux-kernel-exploits](./linux-kernel-exploits/) |
| 🌳 **merkle** | Efficient Merkle hash tree implementation using SHA-256 | ✅ Complete | [research/merkle](./research/merkle/) |
| 🚧 **protocols** | Network protocol analysis (in development) | 🚧 In Progress | [research/protocols](./research/protocols/) |
| 📋 **vulns** | Vulnerability research & proofs of concept (planned) | 📋 Planned | [research/vulns](./research/vulns/) |
| 📋 **hash_cracker** | Multi-algorithm password cracking tool | 📋 Planned | [tools/hash_cracker](./tools/hash_cracker/) |
| 📋 **log_analyzer** | Security log correlation and anomaly detection | 📋 Planned | [tools/log_analyzer](./tools/log_analyzer/) |
| 📋 **web_fuzzer** | Web directory/parameter fuzzer | 📋 Planned | [tools/web_fuzzer](./tools/web_fuzzer/) |
| 📋 **packet_sniffer** | Network packet sniffer | 📋 Planned | [tools/packet_sniffer](./tools/packet_sniffer/) |

> ⚠️ **Important:** All tools are designed for **educational and authorized security testing only**. Always ensure proper authorization before use.

### 🔐 Discord Security Audit Bot

Automated Discord server security assessment and monitoring bot built with [Serenity](https://github.com/serenity-rs/serenity).

**Key Features:**
- Automated security audits and vulnerability scanning
- Manual security assessments and compliance checks
- Security best practices checklist and recommendations
- Real-time monitoring and alert capabilities

**Quick Start:**
```bash
# Set up bot token
echo "DISCORD_TOKEN=your_bot_token_here" > tools/discord_audit_bot/.env

# Run the bot
cargo run -p discord_audit_bot
```

### 🌐 Packet Pattern Matcher & Fuzzer

Advanced pattern matching tool for streaming packet data using the Knuth-Morris-Pratt (KMP) algorithm for network traffic analysis and security testing.

**Features:**
- High-performance pattern matching in network streams
- Fuzzing capabilities for payload analysis
- Real-time packet processing with async Rust

### 📡 Wi-Fi Audit Tool

Passive Wi-Fi network auditing tool for authorized penetration testing and security assessment.

**Capabilities:**
- 802.11 management frame analysis (Beacon/Probe Request/Response)
- SSID inventory and BSSID mapping
- Client device reconnaissance and probing analysis
- Monitor mode packet capture with custom BPF filters

**Usage:**
```bash
# Basic network discovery
sudo cargo run -p wifi_audit -- --iface wlan0mon

# With client monitoring
sudo cargo run -p wifi_audit -- --iface wlan0mon --list-clients
```

## 🚧 Research Modules

### [🔐 Cryptography Toolkit](./research/crypto/)
Educational implementations of cryptographic algorithms in Rust.

**Implemented:**
- **Classical Ciphers**: Caesar cipher, Vigenère cipher
- **Modern Ciphers**: XOR cipher, basic Feistel network
- **Asymmetric Crypto**: Basic RSA implementation
- **Hash Functions**: FNV-1a based hash, hash chaining
- **Utilities**: Key generation, padding schemes

**Example:**
```bash
cargo run -p crypto --example demo
```

### [🌳 Merkle Tree Library](./research/merkle/)
Efficient Merkle hash tree implementation using SHA-256 for data integrity verification.

**Features:**
- Tree construction and management
- Inclusion proof generation
- Proof verification for data integrity
- Membership validation
- Optimized for performance

**Usage:**
```bash
cargo test -p merkle
```

### 🔬 In Development

#### [🎯 Packet Pattern Matcher](./tools/packet-match-fuzz/)
Advanced pattern matching tool for streaming packet data using the Knuth-Morris-Pratt (KMP) algorithm.

**Planned Features:**
- Real-time packet stream analysis
- Pattern matching in network traffic
- Fuzzing pattern detection
- Performance-optimized streaming algorithms

## 📁 Project Structure

```
rust-security-suminworld/
├── tools/                   # Individual security tools (Rust crates)
│   ├── discord_audit_bot/   # ✅ Discord security audit bot
│   ├── packet-match-fuzz/   # ✅ KMP-based pattern matching & fuzzing
│   ├── wifi_audit/          # ✅ Passive Wi-Fi auditing tool
│   ├── hash_cracker/        # 📋 Password & hash cracking tool
│   ├── log_analyzer/        # 📋 Security log analysis tool
│   ├── web_fuzzer/          # 📋 Web directory/parameter fuzzer
│   └── packet_sniffer/      # 📋 Network packet sniffer
├── research/                # Security research modules
│   ├── crypto/              # ✅ Cryptographic algorithms & implementations
│   ├── merkle/              # ✅ Merkle tree implementation
│   ├── protocols/           # 🚧 Network protocol analysis
│   └── vulns/               # 📋 Vulnerability research & PoCs
├── linux-kernel-exploits/   # ✅ Kernel exploitation labs & CVE research
├── PoCs/                    # Proof-of-concepts for attacks (e.g., cache side channels)
│   └── cache/               # Flush+Reload timing attack PoC (C code)
├── docs/                    # Documentation and learning resources
│   ├── learning_notes.md    # Study notes and progress logs
│   ├── tool_usage.md        # Detailed usage guides
│   └── references.md        # Resources and references
├── examples/                # Example usage and test cases
├── LICENSE                  # MIT License
└── README.md                # This file
```

**Legend:** ✅ Complete | 🚧 In Progress | 📋 Planned

## 🚀 Quick Start

### Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs/) (2021 edition or later)
- **System Dependencies**: 
  ```bash
  # Ubuntu/Debian (for Wi-Fi tools)
  sudo apt install -y libpcap-dev build-essential
  
  # macOS
  brew install libpcap
  ```
- **Hardware**: USB Wi-Fi adapter for monitor mode (optional, for wifi_audit)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/sumin-world/rust-security-suminworld.git
   cd rust-security-suminworld
   ```

2. **Build all tools**
   ```bash
   cargo build --release
   ```

3. **Build specific tool**
   ```bash
   cargo build -p discord_audit_bot --release
   cargo build -p wifi_audit --release
   cargo build -p packet-match-fuzz --release
   ```

### Running Tools

#### Discord Security Bot
```bash
# Set up environment
cd tools/discord_audit_bot
echo "DISCORD_TOKEN=your_token" > .env

# Run bot
cargo run -p discord_audit_bot
```

#### Wi-Fi Auditing
```bash
# Setup monitor mode
sudo airmon-ng start wlan0  # Creates wlan0mon

# Basic audit
sudo cargo run -p wifi_audit -- --iface wlan0mon

# With client monitoring
sudo cargo run -p wifi_audit -- --iface wlan0mon --list-clients
```

#### Packet Pattern Fuzzing
```bash
# Run pattern matching tests
cargo run -p packet-match-fuzz --example kmp_demo

# Custom payload fuzzing
cargo run -p packet-match-fuzz -- --pattern "HTTP" --input sample.pcap
```

#### Cryptography Examples
```bash
cargo run -p crypto --example demo
```

## 🔎 Side-Channel Research — Flush+Reload (Cache) PoC

**Warning:** The experimental code in this section is for educational purposes only and must be executed exclusively in a local virtual machine or dedicated experimental equipment, and only in environments with explicit authorization.

Side-channel attacks are techniques that infer secrets from incidental information such as computation time, power consumption, or cache behavior. This repository includes a PoC of **Flush+Reload**, a representative cache-based attack technique. Flush+Reload is a high-resolution, low-noise attack targeting the L3 cache that can determine whether specific memory lines have been accessed, without requiring the attacker and victim to share the same CPU core. This PoC consists of C code located in the `PoCs/cache/` directory.

### Overview

- **Victim Program**: A simulator that repeatedly accesses specific memory indices
- **Attacker Program**: Uses `clflush` and `rdtscp` to measure memory access times, distinguishing cache hits from misses
- **Execution Script**: Runs the victim in the background, saves attacker results to CSV, then terminates the victim

### Running the Experiment

```bash
# Run victim process in background
./PoCs/cache/victim_sim &
VICTIM_PID=$!

# Run attacker and save to CSV
./PoCs/cache/flush_reload_attacker > /tmp/flush_reload_data.csv

# Terminate victim process
kill $VICTIM_PID
```

### Data Format and Interpretation

The CSV format is `iter,cycles` where small values (~1,000 cycles) indicate cache hits and large values (hundreds of thousands of cycles) indicate cache misses or interrupt/context switches.

**Sample Output:**
```csv
iter,cycles
0,158000
1,1000
2,1000
3,155000
4,1000
...
```

Measurements typically form two distinct clusters:
- **Low latency cluster**: Cache hits (victim accessed the memory, data in cache)
- **High latency cluster**: Cache misses or interrupts/context switches

### Analysis Examples

**Statistics:**
```bash
# Count samples
wc -l /tmp/flush_reload_data.csv

# Calculate mean
awk -F, 'NR>1{n++; sum+=$2} END{print "Samples:", n, "Mean:", sum/n}' /tmp/flush_reload_data.csv
```

**Visualization (Python):**
```python
import csv, numpy as np
import matplotlib.pyplot as plt

xs = []
with open('/tmp/flush_reload_data.csv') as f:
    r = csv.reader(f)
    next(r)
    for _, c in r:
        xs.append(int(c))

xs = np.array(xs)
plt.hist(xs, bins=200, log=True)
plt.xlabel('Cycles')
plt.ylabel('Count (log scale)')
plt.title('Flush+Reload Distribution')
plt.yscale('log')
plt.show()
```

### Security Implications

Flush+Reload can be exploited for practical attacks such as tracking AES S-box accesses to extract cryptographic keys. Therefore, defensive techniques should be applied to reduce side-channel leakage, including:
- Constant-time implementations
- Cache partitioning (e.g., Intel CAT)
- Memory access pattern obfuscation

## 🛣️ Roadmap

### Phase 1: Core Tools (Current)
- [x] Discord Security Audit Bot
- [x] Wi-Fi Audit Tool (Monitor Mode)
- [x] Packet Pattern Matcher & Fuzzer
- [x] Basic Cryptographic Toolkit
- [x] Merkle Tree Implementation
- [x] Linux Kernel Exploitation Labs

### Phase 2: Advanced Tools (Next)
- [ ] **Hash Cracker** - Multi-algorithm password analysis
  - Dictionary attacks, rainbow tables
  - GPU acceleration support
  - Custom wordlist generation
- [ ] **Web Fuzzer** - Application security testing
  - Directory and file discovery
  - Parameter fuzzing and injection testing
  - Response analysis and filtering
- [ ] **Log Analyzer** - Security event correlation
  - Multi-format log parsing
  - Anomaly detection algorithms
  - Real-time monitoring dashboards

### Phase 3: Research & Advanced Features
- [ ] **Packet Sniffer** - Network traffic analysis
  - Real-time packet capture
  - Protocol analysis and decoding
  - Traffic pattern recognition
- [ ] **Vulnerability Research** - Security testing
  - Proof-of-concept exploits
  - Fuzzing frameworks
  - Binary analysis tools
- [ ] **Protocol Analysis** - Network security
  - Custom protocol implementations
  - Security assessment tools
  - Traffic manipulation utilities

## 📚 Learning Resources

This project serves as a practical learning platform for:

### 🦀 Rust Concepts
- **Ownership & Borrowing**: Memory safety without garbage collection
- **Async Programming**: Tokio runtime and concurrent operations  
- **Error Handling**: Result types and robust error management
- **Performance**: Zero-cost abstractions and optimization techniques

### 🔒 Security Domains  
- **Network Security**: TCP/IP protocols, scanning techniques
- **Cryptography**: Classical and modern cryptographic algorithms
- **Web Security**: Application testing and vulnerability assessment
- **Digital Forensics**: Log analysis and incident response

### 📖 Documentation

- **[Learning Notes](./docs/learning_notes.md)** - Study progress and key concepts
- **[Tool Usage Guide](./docs/tool_usage.md)** - Detailed examples and best practices  
- **[References](./docs/references.md)** - Curated learning resources and links

## 🤝 Contributing

We welcome contributions! This project is designed for learning, so contributions of all levels are appreciated.

### Ways to Contribute
- 🐛 **Bug Reports**: Found an issue? Please open an issue
- 💡 **Feature Requests**: Have an idea? We'd love to hear it
- 📝 **Documentation**: Improve guides, add examples, fix typos
- 🔧 **Code**: Implement features, fix bugs, optimize performance
- 🎓 **Learning**: Share your learning journey and insights

### Getting Started
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Code Style
- Follow standard Rust formatting (`cargo fmt`)
- Run clippy for linting (`cargo clippy`)
- Ensure all tests pass (`cargo test`)
- Add documentation for public APIs

## 📊 Performance & Security

### Performance Characteristics
- **Discord Bot**: Concurrent async command processing with rate limiting
- **Wi-Fi Audit**: Real-time packet capture and analysis with minimal CPU overhead
- **Crypto Operations**: Focus on educational clarity and correctness over raw speed
- **Memory Usage**: Rust's zero-cost abstractions with minimal heap allocation

### Security Considerations
- **Memory Safety**: Rust's ownership system prevents buffer overflows and use-after-free
- **Input Validation**: Comprehensive validation for all network inputs and user parameters
- **Error Handling**: Graceful failure modes without information leakage
- **Dependency Security**: Regular updates and review of third-party crates

### Testing & Quality Assurance
- **Unit Tests**: Comprehensive test coverage for core functionality
- **Integration Tests**: End-to-end testing of network operations
- **Continuous Integration**: Automated testing across multiple platforms
- **Code Quality**: Clippy linting and rustfmt formatting enforcement

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](./LICENSE) file for details.

**Key Points:**
- ✅ Commercial use allowed
- ✅ Modification and distribution allowed  
- ✅ Private use allowed
- ❗ No warranty or liability provided

## ⚠️ Ethical Use & Disclaimer

**IMPORTANT**: All tools in this repository are intended for:
- 📚 **Educational purposes** - Learning cybersecurity concepts
- 🛡️ **Authorized testing** - Only on systems you own or have explicit permission
- 🔬 **Security research** - Improving defensive capabilities

### Ethical Guidelines
- **Always obtain proper authorization** before testing any systems
- **Respect privacy and confidentiality** of data encountered
- **Use knowledge responsibly** to improve security, not exploit vulnerabilities
- **Follow applicable laws and regulations** in your jurisdiction

### Disclaimer
The authors assume **no liability** for misuse of the provided tools. Users are solely responsible for ensuring their activities comply with applicable laws and regulations.

## 🔗 Links & Resources

### Project Links
- **Repository**: [github.com/sumin-world/rust-security-suminworld](https://github.com/sumin-world/rust-security-suminworld)
- **Issues**: [Report bugs or request features](https://github.com/sumin-world/rust-security-suminworld/issues)
- **Releases**: [Latest releases and versions](https://github.com/sumin-world/rust-security-suminworld/releases)

### Learning Resources
- **[The Rust Book](https://doc.rust-lang.org/book/)** - Official Rust documentation
- **[Rustlings](https://github.com/rust-lang/rustlings)** - Interactive Rust exercises
- **[Tokio Tutorial](https://tokio.rs/tokio/tutorial)** - Async Rust programming
- **[OWASP](https://owasp.org/)** - Web application security guidelines
- **[Rust Security](https://github.com/rust-secure-code/safety-dance)** - Secure coding practices

### Dependencies & Credits
- **[Tokio](https://tokio.rs/)** - Async runtime for Rust
- **[Serenity](https://github.com/serenity-rs/serenity)** - Discord bot library
- **[clap](https://github.com/clap-rs/clap)** - Command line argument parser
- **[serde](https://serde.rs/)** - Serialization framework
- **[sha2](https://github.com/RustCrypto/hashes)** - SHA-2 hash functions

---

<div align="center">
<a href="https://github.com/sumin-world/rust-security-suminworld">⭐ Star</a> • 
<a href="https://github.com/sumin-world/rust-security-suminworld/issues">Issues</a> • 
<a href="https://github.com/sumin-world/rust-security-suminworld/releases">Releases</a>
</div>

---

> ⚠️ See docs/LEGAL_NOTICE.md for legal and ethical guidance. Run experiments only on hardware you own or have explicit permission to test.
