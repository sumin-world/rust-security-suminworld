# rust-security-suminworld 🦀🔒

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/sumin-world/rust-security-suminworld/actions/workflows/ci.yml/badge.svg)](https://github.com/sumin-world/rust-security-suminworld/actions)
[![GitHub last commit](https://img.shields.io/github/last-commit/sumin-world/rust-security-suminworld)](https://github.com/sumin-world/rust-security-suminworld/commits/main)
[![GitHub stars](https://img.shields.io/github/stars/sumin-world/rust-security-suminworld?style=social)](https://github.com/sumin-world/rust-security-suminworld/stargazers)

> **Rust-based Security Research Lab** — Passive Wi-Fi auditing, packet fuzzing, Discord security bots, cryptography toolkit, and kernel-level research, all written in safe, high-performance Rust.

---

## 🔖 Topics

`rust` · `security` · `cybersecurity` · `wifi` · `wireless-security` · `penetration-testing` · `802.11` · `pcap` · `cryptography` · `merkle-tree` · `fuzzing` · `discord-bot`

---

## 🎯 Overview

**rust-security-suminworld** is a Cargo workspace containing practical security tools and educational research modules. Every crate compiles with **zero warnings**, passes **`cargo clippy`** cleanly, and ships with unit + integration tests (**64 tests** total).

### Highlights

| | |
|---|---|
| 🚀 **High Performance** | Async I/O via Tokio, zero-cost abstractions |
| 🛡️ **Memory Safe** | Ownership & borrowing eliminate buffer overflows and use-after-free |
| 🧪 **Well Tested** | 64 tests across all workspace members |
| 📐 **Clean Code** | 0 compiler warnings, 0 clippy lints, `cargo fmt` enforced |
| 🧩 **Modular** | Each tool is an independent crate — build and run individually |

---

## 📦 Workspace Members

| Crate | Type | Description |
|-------|------|-------------|
| [`port_scanner`](./tools/port_scanner/) | 🔧 Tool | High-performance async TCP port scanner (Tokio + Semaphore) |
| [`discord_audit_bot`](./tools/discord_audit_bot/) | 🔧 Tool | Discord server security audit bot (Serenity) |
| [`packet-match-fuzz`](./tools/packet-match-fuzz/) | 🔧 Tool | KMP pattern matcher & mutation fuzzer for packet payloads |
| [`wifi_audit`](./tools/wifi_audit/) | 🔧 Tool | Passive 802.11 Wi-Fi auditor (Beacon / Probe analysis) |
| [`crypto`](./research/crypto/) | 🔬 Research | Educational cryptography: Caesar, Vigenère, XOR, Feistel, RSA, FNV-1a |
| [`merkle`](./research/merkle/) | 🔬 Research | SHA-256 Merkle tree with domain-separated hashing & inclusion proofs |
| [`kernel-features`](./research/kernel-features/) | 🔬 Research | Rust-for-Linux language features study (Field Projection, In-place Init, Arbitrary Self Types) |

> ⚠️ **All tools are for educational and authorized security testing only.**

---

## 🚀 Quick Start

### Prerequisites

- **Rust ≥ 1.75** — install via [rustup](https://rustup.rs/)
- **libpcap** (for `wifi_audit`):
  ```bash
  # Ubuntu / Debian
  sudo apt install -y libpcap-dev build-essential
  # macOS
  brew install libpcap
  ```

### Build & Test

```bash
git clone https://github.com/sumin-world/rust-security-suminworld.git
cd rust-security-suminworld

# Build everything
cargo build --release

# Run all 64 tests
cargo test

# Lint check (should produce 0 warnings)
cargo clippy --all-targets
```

### Run Individual Tools

```bash
# Port scanner
cargo run -p port_scanner -- 192.168.1.1 --range 1-1024 --fast

# Discord bot (set token first)
cp .env.example tools/discord_audit_bot/.env   # then fill in DISCORD_TOKEN
cargo run -p discord_audit_bot

# Wi-Fi audit (requires monitor mode + root)
sudo cargo run -p wifi_audit -- --iface wlan0mon

# Crypto demo (Caesar, Vigenère, XOR, Feistel, FNV, entropy)
cargo run -p research-crypto --example demo

# Packet fuzzing library
cargo test -p packet-match-fuzz
```

---

## 🔧 Tools

### Port Scanner

Async TCP port scanner using Tokio with configurable concurrency and timeout.

```bash
# Scan common ports
cargo run -p port_scanner -- scanme.nmap.org -p 22,80,443

# Fast scan (top 1024 ports, 1024 concurrent, 200ms timeout)
cargo run -p port_scanner -- 10.0.0.1 --fast
```

**Tests:** 19 unit tests covering `parse_range`, `parse_ports_list`, `dedup_sort`, `preview_ports`.

### Discord Audit Bot

Modular Discord security bot built with [Serenity](https://github.com/serenity-rs/serenity). Refactored from a single 1,191-line file into five clean modules:

| Module | Responsibility |
|--------|---------------|
| `main.rs` | Entry point & Serenity client setup |
| `models.rs` | `SecurityReport`, `SecurityLevel`, `AppState` structs |
| `scanner.rs` | `SecurityScanner` — audit logic & report formatting |
| `handler.rs` | `EventHandler` — Discord command dispatch |
| `helpers.rs` | Embed builders, `account_age_days` utility |

### Packet-Match-Fuzz

Full KMP-based pattern-matching and mutation-fuzzing library for packet payloads:

| Module | Purpose |
|--------|---------|
| `kmp.rs` | KMP string matcher — `find_all`, `find_first`, `contains` |
| `stream.rs` | Streaming matcher that retains state across packet chunks |
| `fuzz.rs` | Mutation fuzzer (BitFlip, ByteReplace, ByteInsert, ByteDelete, ChunkShuffle) |

**Tests:** 15 unit tests + 1 doc-test.

### Wi-Fi Audit

Passive 802.11 auditing tool — captures Beacon frames, Probe Requests, and Probe Responses via `libpcap` in monitor mode.

```bash
sudo cargo run -p wifi_audit -- --iface wlan0mon --list-clients
```

---

## 🔬 Research

### Cryptography Toolkit (`research/crypto`)

Educational implementations with comprehensive tests:

| Module | Algorithms |
|--------|-----------|
| `classical.rs` | Caesar cipher, Vigenère cipher |
| `symmetric.rs` | XOR cipher, Feistel network |
| `asymmetric.rs` | RSA (Miller–Rabin, modular exponentiation) |
| `hash.rs` | FNV-1a hash, hash-chain with reduction |
| `utils.rs` | Hex encoding, Shannon entropy, random BigUint |

**Tests:** 18 unit tests. **Demo:** `cargo run -p research-crypto --example demo`

### Merkle Tree (`research/merkle`)

SHA-256 Merkle tree with **domain-separated hashing** (leaf `0x00` prefix vs internal `0x01` prefix) for second-preimage resistance.

- `from_leaves()` — build from arbitrary byte slices
- `root()` — get the 32-byte root hash
- `proof(index)` — generate inclusion proof
- `verify(root, leaf, proof, index)` — static verification
- `leaf_count()` — number of leaves

**Tests:** 7 tests (4 unit + 3 integration).

### Kernel Features Study (`research/kernel-features`)

Executable examples exploring Rust language features needed for Linux kernel development:

| Example | Feature |
|---------|---------|
| `field_projection` | Struct field projection through smart pointers |
| `inplace_init` | In-place initialization (avoiding large stack copies) |
| `smart_pointers` | Arbitrary self types for custom smart pointers |
| `limitations` | Current limitations & development timeline |

**Tests:** 4 unit tests. **Docs:** [`research/kernel-features/docs/`](./research/kernel-features/docs/)

---

## 🔎 Side-Channel Research — Flush+Reload PoC

> ⚠️ **Educational only.** Run exclusively on hardware you own.

The `poCs/cache/` directory contains a C-based **Flush+Reload** cache side-channel proof-of-concept:

| File | Role |
|------|------|
| `victim_sim.c` | Victim: repeatedly accesses a probe array indexed by a secret byte |
| `flush_reload_attacker.c` | Attacker: uses `clflush` + `rdtscp` to measure access times |
| `flush_reload_attacker_csv.c` | Attacker variant: outputs `iter,cycles` CSV for analysis |

```bash
# Compile
gcc -O2 -o victim_sim poCs/cache/victim_sim.c
gcc -O2 -o attacker   poCs/cache/flush_reload_attacker_csv.c

# Run
./victim_sim &
VICTIM_PID=$!
./attacker > /tmp/flush_reload_data.csv
kill $VICTIM_PID
```

Cache hits (~1,000 cycles) vs cache misses (>100,000 cycles) reveal whether the victim accessed the targeted memory line.

---

## 📁 Project Structure

```
rust-security-suminworld/
├── Cargo.toml               # Workspace manifest (shared deps, profiles)
├── tools/
│   ├── port_scanner/        # Async TCP port scanner
│   ├── discord_audit_bot/   # Discord security audit bot (5 modules)
│   ├── packet-match-fuzz/   # KMP matcher + mutation fuzzer
│   └── wifi_audit/          # Passive 802.11 auditor
├── research/
│   ├── crypto/              # Educational cryptography toolkit
│   ├── merkle/              # SHA-256 Merkle tree
│   └── kernel-features/     # Rust-for-Linux feature study
├── poCs/
│   └── cache/               # Flush+Reload side-channel PoC (C)
├── docs/
│   ├── learning_notes.md    # Study notes
│   └── LEGAL_NOTICE.md      # Legal & ethical guidance
├── .github/workflows/ci.yml # CI pipeline
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
└── LICENSE                  # MIT
```

---

## 📊 Code Quality

| Metric | Value |
|--------|-------|
| Compiler warnings | **0** |
| Clippy lints | **0** |
| Test count | **64** (all passing) |
| Test failures | **0** |
| Formatting | `cargo fmt` enforced |

### Test Breakdown

| Crate | Tests |
|-------|------:|
| `port_scanner` | 19 |
| `crypto` | 18 |
| `packet-match-fuzz` | 16 |
| `merkle` | 7 |
| `kernel-features` | 4 |
| **Total** | **64** |

---

## 🛣️ Roadmap

### ✅ Phase 1 — Core (Complete)
- [x] Async TCP port scanner with CLI
- [x] Discord security audit bot (modular architecture)
- [x] KMP packet pattern matcher & mutation fuzzer
- [x] Passive Wi-Fi audit tool (802.11)
- [x] Educational cryptography toolkit
- [x] SHA-256 Merkle tree with domain separation
- [x] Rust-for-Linux kernel features study
- [x] Flush+Reload cache side-channel PoC

### 🔜 Phase 2 — Advanced Tools
- [ ] **Hash Cracker** — dictionary attacks, rainbow tables
- [ ] **Web Fuzzer** — directory discovery, parameter injection
- [ ] **Log Analyzer** — multi-format parsing, anomaly detection
- [ ] **Packet Sniffer** — real-time protocol decoding

---

## 🤝 Contributing

Contributions of all levels are welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md) for details.

```bash
# Development workflow
cargo fmt --all             # Format
cargo clippy --all-targets  # Lint
cargo test                  # Test
```

Please ensure your PR introduces **zero new warnings** and includes tests for new functionality.

---

## ⚠️ Ethical Use & Disclaimer

All tools are intended **exclusively** for:
- 📚 **Educational purposes** — learning cybersecurity concepts
- 🛡️ **Authorized testing** — only on systems you own or have explicit permission to test
- 🔬 **Security research** — improving defensive capabilities

The authors assume **no liability** for misuse. Users are solely responsible for ensuring compliance with applicable laws. See [LEGAL_NOTICE.md](./docs/LEGAL_NOTICE.md) and [SECURITY.md](./SECURITY.md).

---

## 📄 License

[MIT](./LICENSE) — free for commercial and personal use.

---

## 🔗 Links

| | |
|---|---|
| 📦 Repository | [github.com/sumin-world/rust-security-suminworld](https://github.com/sumin-world/rust-security-suminworld) |
| 🐛 Issues | [Report a bug](https://github.com/sumin-world/rust-security-suminworld/issues) |
| 📖 Rust Book | [doc.rust-lang.org/book](https://doc.rust-lang.org/book/) |
| ⚡ Tokio | [tokio.rs](https://tokio.rs/) |
| 🔐 RustCrypto | [github.com/RustCrypto](https://github.com/RustCrypto) |

---

<div align="center">

**Built with 🦀 Rust — memory-safe, blazingly fast, fearlessly concurrent.**

[⭐ Star](https://github.com/sumin-world/rust-security-suminworld) · [Issues](https://github.com/sumin-world/rust-security-suminworld/issues) · [Releases](https://github.com/sumin-world/rust-security-suminworld/releases)

</div>
