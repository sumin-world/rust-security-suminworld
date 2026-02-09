use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, UserId};
use std::collections::{HashMap, HashSet};
use std::env;

// ── Security report types ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSecurityReport {
    pub server_name: String,
    pub check_timestamp: DateTime<Utc>,
    pub overall_score: u8,
    pub security_level: SecurityLevel,
    pub categories: Vec<SecurityCategory>,
    pub critical_issues: Vec<SecurityIssue>,
    pub recommendations: Vec<SecurityRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Excellent,
    Good,
    Average,
    Poor,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCategory {
    pub name: String,
    pub score: u8,
    pub weight: u8,
    pub checks: Vec<SecurityCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCheck {
    pub name: String,
    pub status: CheckStatus,
    pub description: String,
    pub impact: ImpactLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckStatus {
    Pass,
    Fail,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub title: String,
    pub description: String,
    pub severity: ImpactLevel,
    pub solution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRecommendation {
    pub category: String,
    pub action: String,
    pub priority: Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Immediate,
    High,
    Medium,
    Low,
}

// ── Audit category ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuditCategory {
    PasswordHygiene,
    BrowserSecurity,
    SocialMediaPrivacy,
    DeviceSecurity,
    NetworkSafety,
}

impl std::str::FromStr for AuditCategory {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "password" | "passwordhygiene" | "pw" => Ok(Self::PasswordHygiene),
            "browser" | "browsersecurity" => Ok(Self::BrowserSecurity),
            "sns" | "social" | "socialmediaprivacy" => Ok(Self::SocialMediaPrivacy),
            "device" | "devicesecurity" => Ok(Self::DeviceSecurity),
            "network" | "networksafety" => Ok(Self::NetworkSafety),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for AuditCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::PasswordHygiene => "PasswordHygiene",
            Self::BrowserSecurity => "BrowserSecurity",
            Self::SocialMediaPrivacy => "SocialMediaPrivacy",
            Self::DeviceSecurity => "DeviceSecurity",
            Self::NetworkSafety => "NetworkSafety",
        };
        write!(f, "{s}")
    }
}

// ── Mutual security audit ──────────────────────────────────────────

pub struct MutualSecurityAudit {
    pub buddy_pairs: HashSet<(UserId, UserId)>,
    /// Per-category audit (reserved for future per-category commands).
    #[allow(dead_code)]
    pub categories: Vec<AuditCategory>,
}

impl MutualSecurityAudit {
    pub fn new() -> Self {
        Self {
            buddy_pairs: HashSet::new(),
            categories: vec![
                AuditCategory::PasswordHygiene,
                AuditCategory::BrowserSecurity,
                AuditCategory::SocialMediaPrivacy,
                AuditCategory::DeviceSecurity,
                AuditCategory::NetworkSafety,
            ],
        }
    }

    fn normalize_pair(a: UserId, b: UserId) -> (UserId, UserId) {
        if a.get() <= b.get() {
            (a, b)
        } else {
            (b, a)
        }
    }

    pub fn add_pair(&mut self, a: UserId, b: UserId) -> bool {
        self.buddy_pairs.insert(Self::normalize_pair(a, b))
    }

    /// Remove a buddy pair (for future `!짝해제` command).
    #[allow(dead_code)]
    pub fn remove_pair(&mut self, a: UserId, b: UserId) -> bool {
        self.buddy_pairs.remove(&Self::normalize_pair(a, b))
    }
}

impl Default for MutualSecurityAudit {
    fn default() -> Self {
        Self::new()
    }
}

// ── Whitelist ──────────────────────────────────────────────────────

/// Trusted bot whitelist (reserved for future bot verification).
#[derive(Default)]
#[allow(dead_code)]
pub struct Whitelist {
    pub trusted_bots: HashSet<u64>,
}

// ── Application state ──────────────────────────────────────────────

pub struct AppState {
    /// Bot whitelist (reserved for future bot verification feature).
    #[allow(dead_code)]
    pub whitelist: Whitelist,
    pub log_channel: Option<ChannelId>,
    pub audit: MutualSecurityAudit,
    pub security_reports: HashMap<u64, Vec<ServerSecurityReport>>,
}

impl AppState {
    pub fn from_env() -> Self {
        let log_channel = env::var("LOG_CHANNEL_ID")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(ChannelId::new);
        Self {
            whitelist: Whitelist::default(),
            log_channel,
            audit: MutualSecurityAudit::new(),
            security_reports: HashMap::new(),
        }
    }
}
