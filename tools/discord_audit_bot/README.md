# 🔐 Discord Audit Bot

Rust와 [Serenity](https://github.com/serenity-rs/serenity) 기반으로 만든 **디스코드 서버 보안 감사 봇**입니다.  
자동/수동 점검으로 서버 보안을 강화하고, 게임처럼 즐기며 학습할 수 있습니다.  

A **Discord security audit bot** built with Rust + [Serenity](https://github.com/serenity-rs/serenity).  
It provides automated & manual scans, security checklists, and gamified challenges to keep your server safe.  

---

## ✨ Features

### 🔍 Automated & Manual Scans
- `!스캔` / `!scan`: Full server security audit with detailed reports
- `!빠른스캔` / `!quickscan`: Quick check (verification level, 2FA, bot ratio, admin roles, etc.)
- `!스캔기록` / `!history`: View recent security audit history
- `!서버점검` / `!manual`: Manual audit guide for collaborative checks

### 🤝 Peer Auditing
- `!짝매칭 @사용자` / `!pair @user`: Create a buddy pair for security audits
- `!내짝` / `!mypair`: Check your current buddy
- `!체크리스트` / `!checklist`: Security checklist (2FA, browser permissions, VPN usage, etc.)

### 🎮 Gamified Security
- `!챌린지` / `!challenge`: Weekly security challenges (e.g., achieve 80+ score, enable 2FA)
- `!실시간점검` / `!realtime`: Real-time audits via voice chat + screen sharing

### ℹ️ Misc
- `!ping`: Bot status check
- `!안전` / `!about`: About the bot
- `!도움말` / `!help`: Show all commands in an embed

---

## 🛠 Tech Stack
- **Language**: Rust 2021
- **Libraries**: [serenity](https://crates.io/crates/serenity), tokio, chrono
- **Core Components**:
  - `AppState`: Stores audit state (pairs, reports, etc.)
  - `SecurityScanner`: Automated security audit logic
  - `EventHandler`: Handles message/member events

---

## 🚀 Getting Started
```bash
# Clone repository
git clone https://github.com/sumin-world/rust-security-lab.git
cd rust-security-lab/tools/discord_audit_bot

# Create .env file
echo "DISCORD_TOKEN=YOUR_TOKEN_HERE" > .env

# Run
   ```bash
   cargo run
   ```
- Invite the bot to your Discord server with message + server management permissions.

## 🛠 Roadmap / Improvements (Planned)
- Clean unused imports (e.g., gateway::Ready)

- Refactor unused struct fields (trusted_bots, audit_categories, whitelist)

- Decide on handling helper functions (cached_guild, account_age_days, etc.)

- Run cargo fix to resolve warnings

- Add visual reports & dashboards (future)

## 📷 Screenshots (Coming Soon 🚧)
- Example of security audit report embed

- Step-by-step screenshots of !스캔 and !빠른스캔 results
