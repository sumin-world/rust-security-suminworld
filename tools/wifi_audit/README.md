# wifi_audit 📡

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../../LICENSE)

> **Passive Wi-Fi Audit Tool for authorized networks**

A lightweight Rust tool for passive analysis of 802.11 management frames (Beacon/Probe Request/Probe Response) captured in monitor mode. Provides quick network inventory and client reconnaissance for authorized penetration testing and network auditing.

**권한 있는 네트워크에서 Wi-Fi 관리 프레임을 수동으로 분석하여 SSID 인벤토리와 클라이언트 정보를 제공하는 도구입니다.**

---

## ✨ Features

### 📊 Network Discovery
- **SSID Inventory** — Discover all broadcasted network names
- **BSSID Mapping** — Track access points per SSID
- **Channel Distribution** — Monitor frequency usage patterns
- **Beacon Statistics** — Frame count and signal analysis

### 🔍 Client Analysis
- **Probe Request Monitoring** — Track client device activity
- **MAC Address Collection** — Device fingerprinting (anonymized)
- **Network Preference Discovery** — Client's preferred networks

### ⚙️ Technical Features
- **Custom BPF Filters** — Fine-tune packet capture
- **Real-time Updates** — Live dashboard with configurable refresh
- **High Performance** — Async Rust with zero-copy packet processing
- **Cross-platform** — Linux/macOS support with monitor mode

---

## 🚀 Quick Start

### Prerequisites

**System Requirements:**
```bash
# Ubuntu/Debian
sudo apt update && sudo apt install -y libpcap-dev build-essential

# macOS (with Homebrew)
brew install libpcap
```

**Wi-Fi Hardware:**
- USB Wi-Fi adapter supporting monitor mode
- Recommended chipsets: Atheros AR9271, Ralink RT5572, Realtek 8812AU

### Installation

```bash
# From project root
cargo build -p wifi_audit --release

# Or install directly
cargo install --path tools/wifi_audit
```

### Monitor Mode Setup

**Linux with airmon-ng:**
```bash
# Install aircrack-ng suite
sudo apt install -y aircrack-ng

# Enable monitor mode
sudo airmon-ng start wlan0    # Creates wlan0mon
```

**For Realtek 8812AU dongles:**
```bash
# Install driver
sudo apt install -y build-essential dkms git
git clone https://github.com/aircrack-ng/rtl8812au.git
cd rtl8812au && sudo make dkms_install
```

---

## 🎯 Usage

### Basic Network Scan
```bash
# Simple SSID discovery
sudo cargo run -p wifi_audit --release -- --iface wlan0mon

# With client probe monitoring
sudo cargo run -p wifi_audit --release -- \
    --iface wlan0mon \
    --list-clients \
    --refresh-secs 10
```

### Advanced Options
```bash
wifi_audit [OPTIONS] --iface <INTERFACE>

OPTIONS:
    -i, --iface <IFACE>        Monitor mode interface (e.g., wlan0mon)
    -f, --filter <BPF>         Custom BPF filter for packet capture
    -m, --max-frames <N>       Stop after N frames (0 = unlimited)
    -r, --refresh-secs <N>     Table refresh interval in seconds [default: 5]
    -c, --list-clients         Show probe request client summary
    -v, --verbose              Enable detailed packet logging
    -h, --help                 Print help information
```

### Custom BPF Filters
```bash
# Default filter (management frames only)
"type mgt and (subtype beacon or subtype probe-req or subtype probe-res)"

# Beacon frames only
sudo wifi_audit --iface wlan0mon --filter "type mgt subtype beacon"

# Specific channel monitoring
sudo wifi_audit --iface wlan0mon --filter "type mgt and channel 6"
```

---

## 📊 Sample Output

### SSID Inventory Table
```
┏━━━━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━┳━━━━━━━━━━━┓
┃ SSID                 ┃ BSSIDs                                ┃ Channels      ┃ Beacons   ┃
┣━━━━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━╋━━━━━━━━━━━┫
┃ HomeNetwork_5G       ┃ 34:12:ab:cd:ef:01                     ┃ 36            ┃ 1,247     ┃
┃ CoffeeShop_Free      ┃ d0:af:98:12:34:56, d0:af:98:12:34:57  ┃ 1, 6, 11      ┃ 3,892     ┃
┃ Corporate_WiFi       ┃ 00:1a:2b:3c:4d:5e                     ┃ 44            ┃ 567       ┃
┃ [Hidden SSID]        ┃ aa:bb:cc:dd:ee:ff                     ┃ 6             ┃ 234       ┃
┗━━━━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━┻━━━━━━━━━━━┛

📡 Active Networks: 4 SSIDs, 6 BSSIDs across 5 channels
⏱️  Capture Time: 00:03:45 | Total Frames: 5,940
```

### Client Activity (with `--list-clients`)
```
┏━━━━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ Client MAC           ┃ Probe Requests    ┃ Probed SSIDs                                    ┃
┣━━━━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃ ab:cd:ef:12:34:56    ┃ 23                ┃ HomeNetwork, Starbucks_WiFi, Airport_Free       ┃
┃ 12:34:56:ab:cd:ef    ┃ 15                ┃ CorporateWiFi, iPhone_Hotspot                   ┃
┃ 98:76:54:32:10:fe    ┃ 8                 ┃ AndroidAP_1234                                  ┃
┗━━━━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

📱 Active Clients: 3 unique devices, 46 total probe requests
```

---

## 🛠️ Hardware Setup Guide

### Recommended USB Wi-Fi Adapters

| **Chipset** | **Model Example** | **Monitor Mode** | **Notes** |
|-------------|-------------------|------------------|-----------|
| **Atheros AR9271** | Alfa AWUS036NHA | ✅ Excellent | Plug-and-play on Linux |
| **Ralink RT5572** | Panda PAU09 | ✅ Good | Stable performance |
| **Realtek 8812AU** | Alfa AWUS036ACS | ✅ Good | Requires driver installation |
| **MediaTek MT7612U** | Panda PAU08 | ⚠️ Limited | Newer kernel support needed |

### VM Passthrough (macOS/Windows hosts)
```bash
# VirtualBox: Enable USB 2.0/3.0 controller
# VMware: Add USB device in VM settings
# Pass USB Wi-Fi adapter to Linux VM for monitor mode support
```

### Troubleshooting

**Interface not found:**
```bash
# Check available interfaces
iwconfig
ip link show

# Ensure monitor mode is active
sudo airmon-ng check kill  # Stop interfering processes
sudo airmon-ng start wlan0
```

**Permission denied:**
```bash
# Run with sudo for raw socket access
sudo wifi_audit --iface wlan0mon
```

**No packets captured:**
```bash
# Verify monitor mode
iwconfig wlan0mon  # Should show Mode:Monitor

# Test with tcpdump
sudo tcpdump -i wlan0mon -c 10 -nn
```

---

## 🏗️ Architecture

### Project Structure
```
tools/wifi_audit/
├── Cargo.toml              # Dependencies and metadata
├── README.md               # This file
└── src/
    ├── main.rs             # CLI interface and main loop
    ├── capture.rs          # libpcap integration and packet capture
    ├── parse.rs            # 802.11 frame parsing (radiotap + management)
    ├── report.rs           # Statistics aggregation and table rendering
    └── types.rs            # Data structures and packet definitions
```

### Key Dependencies
```toml
[dependencies]
pcap = "1.3"           # Packet capture library
clap = "4.4"           # Command line argument parsing
tabled = "0.15"        # ASCII table formatting
tokio = "1.35"         # Async runtime for high performance
serde = "1.0"          # Serialization for data structures
anyhow = "1.0"         # Error handling
```

### Performance Characteristics
- **Zero-copy packet processing** using raw byte manipulation
- **Async I/O** with Tokio for non-blocking capture loops
- **Memory efficient** hash maps for MAC address deduplication
- **Real-time updates** with configurable refresh intervals

---

## 🗺️ Roadmap

### Version 0.2.0 (Planned)
- [ ] **Channel Congestion Analysis** — Frames per second by channel
- [ ] **Suspicious AP Detection** — Multiple BSSIDs with same SSID
- [ ] **Signal Strength Mapping** — RSSI tracking and visualization
- [ ] **Export Functionality** — JSON/CSV output for external analysis

### Version 0.3.0 (Future)
- [ ] **OUI Vendor Lookup** — Device manufacturer identification
- [ ] **PCAP File Support** — Offline analysis of captured files  
- [ ] **Web Dashboard** — Real-time browser-based monitoring
- [ ] **Bluetooth LE Scanning** — Extend to IoT device discovery

### Performance Improvements
- [ ] **Multi-threading** — Parallel processing for high-traffic environments
- [ ] **GPU Acceleration** — CUDA support for large-scale analysis
- [ ] **Database Backend** — PostgreSQL integration for historical data

---

## ⚖️ Legal & Ethical Use

### ⚠️ Important Disclaimer

**This tool is designed for authorized security testing and network auditing only.**

### Authorized Use Cases
- ✅ **Your own networks** — Testing personal Wi-Fi security
- ✅ **Client engagements** — Authorized penetration testing
- ✅ **Educational purposes** — Learning 802.11 protocols
- ✅ **Security research** — Academic and professional research

### Prohibited Activities
- ❌ **Unauthorized monitoring** of third-party networks
- ❌ **Active attacks** — This tool is passive only
- ❌ **Data interception** — No decryption or payload analysis
- ❌ **Privacy violations** — Respect others' network privacy

### Legal Compliance
- Always obtain **written authorization** before testing
- Comply with **local laws** regarding wireless monitoring
- Respect **privacy regulations** (GDPR, CCPA, etc.)
- Follow **responsible disclosure** for discovered vulnerabilities

### Technical Limitations
- **Passive monitoring only** — No packet injection or jamming
- **Management frames only** — No data payload interception  
- **Public information** — Only broadcasted network identifiers
- **Anonymized reporting** — No personal data collection

---

## 🤝 Contributing

We welcome contributions from the security community!

### How to Contribute
1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/awesome-feature`)
3. **Implement** your changes with tests
4. **Document** your code and update README if needed
5. **Test** on multiple platforms and hardware
6. **Submit** a pull request with detailed description

### Development Setup
```bash
# Clone and build
git clone https://github.com/sumin-world/rust-security-suminworld.git
cd rust-security-suminworld/tools/wifi_audit
cargo build

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt --check
```

### Code Style
- Follow **Rust conventions** with `rustfmt`
- Add **comprehensive tests** for new features
- Include **inline documentation** for public APIs
- Use **meaningful variable names** and comments

---

## 📚 References & Resources

### 802.11 Protocol Documentation
- **[IEEE 802.11 Standard](https://ieeexplore.ieee.org/document/9363693)** — Official Wi-Fi specification
- **[Wireshark 802.11 Analysis](https://wiki.wireshark.org/802.11)** — Packet analysis guide
- **[Aircrack-ng Documentation](https://www.aircrack-ng.org/documentation.html)** — Monitor mode setup

### Security Research
- **[OWASP Wireless Security](https://owasp.org/www-project-iot-security-top-10/)** — Best practices
- **[NIST Cybersecurity Framework](https://www.nist.gov/cyberframework)** — Security guidelines
- **[Wireless Security Testing Guide](https://github.com/OWASP/wstg)** — Penetration testing

### Rust Resources
- **[The Rust Book](https://doc.rust-lang.org/book/)** — Official Rust documentation
- **[Tokio Tutorial](https://tokio.rs/tokio/tutorial)** — Async programming in Rust
- **[pcap-rs Documentation](https://docs.rs/pcap/)** — Packet capture library

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](../../LICENSE) file for details.

**You are free to:**
- ✅ Use commercially and personally
- ✅ Modify and distribute
- ✅ Include in other projects

**With the requirement to:**
- 📝 Include original license text
- 📝 Preserve copyright notices

---

<div align="center">

**For educational and authorized testing purposes only**

[🏠 Back to Main Project](../../README.md) • [🐛 Report Issues](../../issues) • [⭐ Star Project](../../)

</div>
