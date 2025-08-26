# sum-rust-secu-lab 🦀🔒

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub last commit](https://img.shields.io/github/last-commit/sumin-world/sum-rust-secu-lab)](https://github.com/sumin-world/sum-rust-secu-lab/commits/main)
[![GitHub stars](https://img.shields.io/github/stars/sumin-world/sum-rust-secu-lab?style=social)](https://github.com/sumin-world/sum-rust-secu-lab/stargazers)

> **Security tools and research built with Rust**

A comprehensive collection of Rust-based cybersecurity tools and research projects designed for learning and demonstrating security concepts. This repository showcases Rust's capabilities in building high-performance, memory-safe security applications.

## 🎯 Overview

**sum-rust-secu-lab** is a personal security laboratory focused on learning cybersecurity concepts through hands-on Rust development. The project demonstrates Rust's strengths in security applications: memory safety, fearless concurrency, and zero-cost abstractions.

### 🌟 Key Features

- **High Performance**: Leverages Rust's zero-cost abstractions and async capabilities
- **Memory Safety**: Eliminates common security vulnerabilities through Rust's ownership system  
- **Concurrent Processing**: Utilizes Tokio for high-performance async operations
- **Educational Focus**: Comprehensive documentation and learning resources
- **Modular Architecture**: Clean separation of tools, research, and documentation

## 🛠️ Current Tools

### ✅ Implemented

#### [🔍 Port Scanner](./tools/port_scanner/)
High-performance asynchronous TCP port scanner with concurrent connection handling.

**Features:**
- Async TCP scanning using [Tokio](https://tokio.rs/)
- Customizable concurrency levels and timeouts
- Support for port ranges and individual port specification
- Fast scanning modes for common ports
- Detailed connection status reporting

**Usage:**
```bash
# Scan specific ports
cargo run -p port_scanner -- 192.168.1.1 -p 22,80,443,8080

# Scan port range with high concurrency
cargo run -p port_scanner -- example.com --range 1-1000 --fast

# Custom timeout and concurrency
cargo run -p port_scanner -- target.com -p 80,443 --timeout 3 --threads 100
```

#### [🤖 Discord Security Audit Bot](./tools/discord_audit_bot/)
Automated Discord server security assessment and monitoring bot built with [Serenity](https://github.com/serenity-rs/serenity).

**Features:**
- Automated security audits for Discord servers
- Manual security checks and assessments
- Security checklist and best practices
- Peer audit pairing system
- Gamified security challenges
- Real-time monitoring capabilities

**Setup:**
```bash
# Create .env file with bot token
echo "DISCORD_TOKEN=your_bot_token_here" > tools/discord_audit_bot/.env

# Run the bot
cargo run -p discord_audit_bot
```

**Commands:**
- `!scan` - Full security audit
- `!quickscan` - Basic security checks  
- `!checklist` - Security best practices
- `!history` - Recent audit reports
- `!help` - Command documentation

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

#### [🎯 Packet Pattern Matcher](./research/packet-match-fuzz/)
Advanced pattern matching tool for streaming packet data using the Knuth-Morris-Pratt (KMP) algorithm.

**Planned Features:**
- Real-time packet stream analysis
- Pattern matching in network traffic
- Fuzzing pattern detection
- Performance-optimized streaming algorithms

## 📁 Project Structure

- **[tools/](./tools/)** — Individual security tools (Rust crates)
  - **[port_scanner/](./tools/port_scanner/)** — ✅ Async TCP port scanner 
  - **[discord_audit_bot/](./tools/discord_audit_bot/)** — ✅ Discord security audit bot
  - **packet-match-fuzz/** — 🚧 KMP-based packet matching
  - **hash_cracker/** — 📋 Password & hash cracking tool
  - **log_analyzer/** — 📋 Security log analysis tool
  - **web_fuzzer/** — 📋 Web directory/parameter fuzzer
  - **packet_sniffer/** — 📋 Network packet sniffer
- **[research/](./research/)** — Security research modules
  - **[crypto/](./research/crypto/)** — ✅ Cryptographic algorithms
  - **[merkle/](./research/merkle/)** — ✅ Merkle tree implementation
  - **protocols/** — 🚧 Network protocol analysis
  - **vulns/** — 📋 Vulnerability research & PoCs
- **[docs/](./docs/)** — Documentation & learning resources
  - **[learning_notes.md](./docs/learning_notes.md)**
  - **[tool_usage.md](./docs/tool_usage.md)**
  - **[references.md](./docs/references.md)**
- **[examples/](./examples/)** — Example usage and test cases
- **[LICENSE](./LICENSE)** — MIT License
- **[README.md](./README.md)** — This file

**Legend:** ✅ Complete | 🚧 In Progress | 📋 Planned

## 🚀 Quick Start

### Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs/) (2021 edition or later)
- **Cargo**: Comes with Rust installation
- **Internet connection**: For dependency resolution

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/sumin-world/sum-rust-secu-lab.git
   cd sum-rust-secu-lab
   ```

2. **Build all tools**
   ```bash
   cargo build --release
   ```

3. **Build specific tool**
   ```bash
   cargo build -p port_scanner --release
   ```

### Running Tools

#### Port Scanner
```bash
# Basic usage
cargo run -p port_scanner -- --help

# Scan common ports
cargo run -p port_scanner -- 192.168.1.1 -p 22,80,443

# Fast scan of top 1000 ports
cargo run -p port_scanner -- example.com --fast
```

#### Discord Bot
```bash
# Set up environment
cd tools/discord_audit_bot
echo "DISCORD_TOKEN=your_token" > .env

# Run bot
cargo run -p discord_audit_bot
```

#### Crypto Examples
```bash
cargo run -p crypto --example demo
```

## 🛣️ Roadmap

### Phase 1: Core Tools (Current)
- [x] Async TCP Port Scanner
- [x] Discord Security Audit Bot  
- [x] Basic Cryptographic Toolkit
- [x] Merkle Tree Implementation
- [ ] KMP Packet Pattern Matcher

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
- **Port Scanner**: Concurrent async scanning with configurable timeout and thread limits
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
- **Repository**: [github.com/sumin-world/sum-rust-secu-lab](https://github.com/sumin-world/sum-rust-secu-lab)
- **Issues**: [Report bugs or request features](https://github.com/sumin-world/sum-rust-secu-lab/issues)
- **Releases**: [Latest releases and versions](https://github.com/sumin-world/sum-rust-secu-lab/releases)

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
<a href="https://github.com/sumin-world/sum-rust-secu-lab">⭐ Star</a> • 
<a href="https://github.com/sumin-world/sum-rust-secu-lab/issues">Issues</a> • 
<a href="https://github.com/sumin-world/sum-rust-secu-lab/releases">Releases</a>
</div>
