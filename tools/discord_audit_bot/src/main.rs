use serenity::all::GuildRef;
use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateMessage},
    model::{
        channel::Message,
        gateway::Ready,
        guild::{ExplicitContentFilter, Guild, Member, MfaLevel, VerificationLevel},
        id::{ChannelId, GuildId, UserId},
        prelude::*,
        prelude::{Presence, OnlineStatus},
    },
    prelude::*,
};

use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, collections::HashSet, env};
use tokio::sync::RwLock;

// =====================
// 보안 검사 구조체들
// =====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSecurityReport {
    pub server_name: String,
    pub check_timestamp: DateTime<Utc>,
    pub overall_score: u8, // 0-100
    pub security_level: SecurityLevel,
    pub categories: Vec<SecurityCategory>,
    pub critical_issues: Vec<SecurityIssue>,
    pub recommendations: Vec<SecurityRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Excellent, // 90-100점 🟢
    Good,      // 70-89점  🔵
    Average,   // 50-69점  🟡
    Poor,      // 30-49점  🟠
    Critical,  // 0-29점   🔴
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
    Pass,    // ✅ 통과
    Fail,    // ❌ 실패
    Warning, // ⚠️ 경고
    Info,    // ℹ️ 정보
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    Critical, // 🚨 치명적
    High,     // ⚠️ 높음
    Medium,   // 📋 보통
    Low,      // 💡 낮음
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
    Immediate, // 즉시
    High,      // 높음
    Medium,    // 보통
    Low,       // 낮음
}

// =====================
// 기존 데이터 타입들
// =====================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AuditCategory {
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
        write!(f, "{}", s)
    }
}

#[derive(Default)]
struct Whitelist {
    trusted_bots: HashSet<u64>,
}

struct MutualSecurityAudit {
    buddy_pairs: HashSet<(UserId, UserId)>,
    audit_categories: Vec<AuditCategory>,
}

impl MutualSecurityAudit {
    fn new_default() -> Self {
        Self {
            buddy_pairs: HashSet::new(),
            audit_categories: vec![
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
    fn add_pair(&mut self, a: UserId, b: UserId) -> bool {
        let pair = Self::normalize_pair(a, b);
        self.buddy_pairs.insert(pair)
    }
    fn remove_pair(&mut self, a: UserId, b: UserId) -> bool {
        let pair = Self::normalize_pair(a, b);
        self.buddy_pairs.remove(&pair)
    }
}

struct AppState {
    whitelist: Whitelist,
    log_channel: Option<ChannelId>,
    audit: MutualSecurityAudit,
    security_reports: HashMap<u64, Vec<ServerSecurityReport>>,
}

impl AppState {
    fn from_env() -> Self {
        let log_channel = env::var("LOG_CHANNEL_ID")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(ChannelId::new);
        Self {
            whitelist: Whitelist::default(),
            log_channel,
            audit: MutualSecurityAudit::new_default(),
            security_reports: HashMap::new(),
        }
    }
}

static STATE: Lazy<RwLock<AppState>> = Lazy::new(|| RwLock::new(AppState::from_env()));

// =====================
// 보안 스캐너
// =====================
struct SecurityScanner;

impl SecurityScanner {
    async fn perform_security_audit(ctx: &Context, guild: &Guild) -> ServerSecurityReport {
        let mut report = ServerSecurityReport {
            server_name: guild.name.clone(),
            check_timestamp: Utc::now(),
            overall_score: 0,
            security_level: SecurityLevel::Critical,
            categories: Vec::new(),
            critical_issues: Vec::new(),
            recommendations: Vec::new(),
        };

        // 각 보안 카테고리 점검
        report
            .categories
            .push(Self::check_permissions_security(guild).await);
        report
            .categories
            .push(Self::check_moderation_settings(guild).await);
        report
            .categories
            .push(Self::check_role_security(guild).await);
        report
            .categories
            .push(Self::check_bot_security(ctx, guild).await);

        // 전체 점수 계산
        report.overall_score = Self::calculate_overall_score(&report.categories);
        report.security_level = Self::determine_security_level(report.overall_score);

        // 치명적 이슈 식별
        report.critical_issues = Self::identify_critical_issues(&report.categories);

        // 개선 권장사항 생성
        report.recommendations = Self::generate_recommendations(&report.categories);

        report
    }

    async fn check_permissions_security(guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        // @everyone 역할의 위험한 권한 확인
        if let Some(everyone_role) = guild.roles.values().find(|r| r.name == "@everyone") {
            let dangerous_perms = [
                (everyone_role.permissions.administrator(), "관리자 권한"),
                (everyone_role.permissions.manage_guild(), "서버 관리"),
                (everyone_role.permissions.manage_roles(), "역할 관리"),
                (everyone_role.permissions.manage_channels(), "채널 관리"),
                (everyone_role.permissions.kick_members(), "멤버 추방"),
                (everyone_role.permissions.ban_members(), "멤버 차단"),
                (everyone_role.permissions.mention_everyone(), "전체 멘션"),
            ];

            let mut dangerous_count = 0;
            for (has_perm, perm_name) in dangerous_perms {
                if has_perm {
                    dangerous_count += 1;
                    checks.push(SecurityCheck {
                        name: format!("@everyone {}", perm_name),
                        status: CheckStatus::Fail,
                        description: format!(
                            "@everyone 역할이 위험한 '{}' 권한을 가지고 있습니다",
                            perm_name
                        ),
                        impact: ImpactLevel::Critical,
                    });
                }
            }

            if dangerous_count == 0 {
                checks.push(SecurityCheck {
                    name: "@everyone 권한".to_string(),
                    status: CheckStatus::Pass,
                    description: "@everyone 역할에 안전한 권한만 설정되어 있습니다".to_string(),
                    impact: ImpactLevel::Low,
                });
            } else {
                score = score.saturating_sub(dangerous_count * 25);
            }
        }

        // 관리자 권한을 가진 역할 수 확인
        let admin_roles: Vec<_> = guild
            .roles
            .values()
            .filter(|role| role.permissions.administrator())
            .collect();

        if admin_roles.len() > 3 {
            checks.push(SecurityCheck {
                name: "관리자 역할 수".to_string(),
                status: CheckStatus::Warning,
                description: format!(
                    "{}개의 역할이 관리자 권한을 가지고 있습니다",
                    admin_roles.len()
                ),
                impact: ImpactLevel::Medium,
            });
            score -= 15;
        } else {
            checks.push(SecurityCheck {
                name: "관리자 역할 수".to_string(),
                status: CheckStatus::Pass,
                description: "적절한 수의 관리자 역할이 설정되어 있습니다".to_string(),
                impact: ImpactLevel::Low,
            });
        }

        SecurityCategory {
            name: "권한 보안".to_string(),
            score,
            weight: 35,
            checks,
        }
    }

    async fn check_moderation_settings(guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        // 인증 레벨 확인
        match guild.verification_level {
            VerificationLevel::None => {
                checks.push(SecurityCheck {
                    name: "인증 레벨".to_string(),
                    status: CheckStatus::Fail,
                    description: "인증 레벨이 '없음'으로 설정되어 있습니다".to_string(),
                    impact: ImpactLevel::High,
                });
                score -= 25;
            }
            VerificationLevel::Low => {
                checks.push(SecurityCheck {
                    name: "인증 레벨".to_string(),
                    status: CheckStatus::Warning,
                    description: "인증 레벨이 '낮음'입니다".to_string(),
                    impact: ImpactLevel::Medium,
                });
                score -= 10;
            }
            _ => {
                checks.push(SecurityCheck {
                    name: "인증 레벨".to_string(),
                    status: CheckStatus::Pass,
                    description: "적절한 인증 레벨이 설정되어 있습니다".to_string(),
                    impact: ImpactLevel::Low,
                });
            }
        }

        // MFA 요구사항 확인
        if guild.mfa_level == MfaLevel::None {
            checks.push(SecurityCheck {
                name: "2단계 인증".to_string(),
                status: CheckStatus::Fail,
                description: "관리자에게 2단계 인증이 요구되지 않습니다".to_string(),
                impact: ImpactLevel::Critical,
            });
            score -= 30;
        } else {
            checks.push(SecurityCheck {
                name: "2단계 인증".to_string(),
                status: CheckStatus::Pass,
                description: "관리자에게 2단계 인증이 요구됩니다".to_string(),
                impact: ImpactLevel::Low,
            });
        }

        // 명시적 콘텐츠 필터 확인
        match guild.explicit_content_filter {
            ExplicitContentFilter::None => {
                checks.push(SecurityCheck {
                    name: "콘텐츠 필터".to_string(),
                    status: CheckStatus::Fail,
                    description: "명시적 콘텐츠 필터가 비활성화되어 있습니다".to_string(),
                    impact: ImpactLevel::High,
                });
                score -= 20;
            }
            ExplicitContentFilter::WithoutRole => {
                checks.push(SecurityCheck {
                    name: "콘텐츠 필터".to_string(),
                    status: CheckStatus::Warning,
                    description: "일부 멤버만 콘텐츠 필터가 적용됩니다".to_string(),
                    impact: ImpactLevel::Medium,
                });
                score -= 10;
            }
            ExplicitContentFilter::All => {
                checks.push(SecurityCheck {
                    name: "콘텐츠 필터".to_string(),
                    status: CheckStatus::Pass,
                    description: "모든 멤버에게 콘텐츠 필터가 적용됩니다".to_string(),
                    impact: ImpactLevel::Low,
                });
            }
            _ => {
                checks.push(SecurityCheck {
                    name: "콘텐츠 필터".to_string(),
                    status: CheckStatus::Info,
                    description: "콘텐츠 필터 설정을 확인할 수 없습니다".to_string(),
                    impact: ImpactLevel::Low,
                });
            }
        }

        SecurityCategory {
            name: "조정 설정".to_string(),
            score,
            weight: 30,
            checks,
        }
    }

    async fn check_role_security(guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        // 의심스러운 역할명 확인
        let mut suspicious_roles = Vec::new();
        let suspicious_names = ["everyone", "nitro", "admin", "owner"];

        for role in guild.roles.values() {
            for suspicious in &suspicious_names {
                if role.name.to_lowercase().contains(suspicious) && role.name != "@everyone" {
                    suspicious_roles.push(role);
                    break;
                }
            }
        }

        if !suspicious_roles.is_empty() {
            for role in &suspicious_roles {
                checks.push(SecurityCheck {
                    name: format!("의심스러운 역할 '{}'", role.name),
                    status: CheckStatus::Warning,
                    description: "역할명이 의심스럽습니다".to_string(),
                    impact: ImpactLevel::Medium,
                });
            }
            score -= suspicious_roles.len() as u8 * 10;
        } else {
            checks.push(SecurityCheck {
                name: "역할명 검사".to_string(),
                status: CheckStatus::Pass,
                description: "의심스러운 역할명이 발견되지 않았습니다".to_string(),
                impact: ImpactLevel::Low,
            });
        }

        SecurityCategory {
            name: "역할 보안".to_string(),
            score,
            weight: 20,
            checks,
        }
    }

    async fn check_bot_security(_ctx: &Context, guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        let mut bot_count = 0;
        let mut high_perm_bots = 0;

        for (_user_id, member) in &guild.members {
            if member.user.bot {
                bot_count += 1;

                // 멤버의 역할들을 통해 권한 확인
                let mut has_admin = false;
                for role_id in &member.roles {
                    if let Some(role) = guild.roles.get(role_id) {
                        if role.permissions.administrator() || role.permissions.manage_guild() {
                            has_admin = true;
                            break;
                        }
                    }
                }

                if has_admin {
                    high_perm_bots += 1;
                }
            }
        }

        // 봇 비율 확인
        let total_members = guild.members.len();
        if total_members > 0 {
            let bot_ratio = (bot_count * 100) / total_members;

            if bot_ratio > 30 {
                checks.push(SecurityCheck {
                    name: "봇 비율".to_string(),
                    status: CheckStatus::Warning,
                    description: format!("서버의 {}%가 봇입니다", bot_ratio),
                    impact: ImpactLevel::Medium,
                });
                score -= 15;
            } else {
                checks.push(SecurityCheck {
                    name: "봇 비율".to_string(),
                    status: CheckStatus::Pass,
                    description: format!("적절한 봇 비율입니다 ({}%)", bot_ratio),
                    impact: ImpactLevel::Low,
                });
            }
        }

        // 높은 권한을 가진 봇들
        if high_perm_bots > 0 {
            checks.push(SecurityCheck {
                name: "고권한 봇".to_string(),
                status: CheckStatus::Warning,
                description: format!("{}개의 봇이 관리자 권한을 가지고 있습니다", high_perm_bots),
                impact: ImpactLevel::High,
            });
            score -= high_perm_bots as u8 * 15;
        } else if bot_count > 0 {
            checks.push(SecurityCheck {
                name: "봇 권한".to_string(),
                status: CheckStatus::Pass,
                description: "봇들이 적절한 권한을 가지고 있습니다".to_string(),
                impact: ImpactLevel::Low,
            });
        }

        SecurityCategory {
            name: "봇 보안".to_string(),
            score,
            weight: 15,
            checks,
        }
    }

    fn calculate_overall_score(categories: &[SecurityCategory]) -> u8 {
        let total_weight: u8 = categories.iter().map(|c| c.weight).sum();
        if total_weight == 0 {
            return 0;
        }

        let weighted_score: u32 = categories
            .iter()
            .map(|c| c.score as u32 * c.weight as u32)
            .sum();

        (weighted_score / total_weight as u32) as u8
    }

    fn determine_security_level(score: u8) -> SecurityLevel {
        match score {
            90..=100 => SecurityLevel::Excellent,
            70..=89 => SecurityLevel::Good,
            50..=69 => SecurityLevel::Average,
            30..=49 => SecurityLevel::Poor,
            _ => SecurityLevel::Critical,
        }
    }

    fn identify_critical_issues(categories: &[SecurityCategory]) -> Vec<SecurityIssue> {
        let mut critical_issues = Vec::new();

        for category in categories {
            for check in &category.checks {
                if matches!(check.impact, ImpactLevel::Critical)
                    && matches!(check.status, CheckStatus::Fail)
                {
                    critical_issues.push(SecurityIssue {
                        title: check.name.clone(),
                        description: check.description.clone(),
                        severity: check.impact.clone(),
                        solution: Self::get_solution_for_check(&check.name),
                    });
                }
            }
        }

        critical_issues
    }

    fn generate_recommendations(categories: &[SecurityCategory]) -> Vec<SecurityRecommendation> {
        let mut recommendations = Vec::new();

        for category in categories {
            if category.score < 70 {
                let recommendation = match category.name.as_str() {
                    "권한 보안" => SecurityRecommendation {
                        category: category.name.clone(),
                        action: "관리자 권한을 가진 역할 수를 줄이고, @everyone 권한을 검토하세요"
                            .to_string(),
                        priority: Priority::High,
                    },
                    "조정 설정" => SecurityRecommendation {
                        category: category.name.clone(),
                        action: "인증 레벨을 높이고 2단계 인증을 활성화하세요".to_string(),
                        priority: Priority::High,
                    },
                    "봇 보안" => SecurityRecommendation {
                        category: category.name.clone(),
                        action: "불필요한 봇을 제거하고 봇 권한을 최소화하세요".to_string(),
                        priority: Priority::Medium,
                    },
                    _ => SecurityRecommendation {
                        category: category.name.clone(),
                        action: format!("{} 관련 설정을 검토하고 개선하세요", category.name),
                        priority: Priority::Medium,
                    },
                };
                recommendations.push(recommendation);
            }
        }

        recommendations
    }

    fn get_solution_for_check(check_name: &str) -> String {
        match check_name {
            name if name.contains("@everyone") => {
                "서버 설정 → 역할 → @everyone → 위험한 권한들을 비활성화하세요".to_string()
            }
            name if name.contains("2단계 인증") => {
                "서버 설정 → 조정 → 관리 작업에 2단계 인증 요구를 활성화하세요".to_string()
            }
            name if name.contains("인증 레벨") => {
                "서버 설정 → 조정 → 인증 레벨을 '중간' 이상으로 설정하세요".to_string()
            }
            _ => "해당 설정을 검토하고 보안 모범 사례에 따라 수정하세요".to_string(),
        }
    }

    fn format_security_report(report: &ServerSecurityReport) -> Vec<String> {
        let mut parts = Vec::new();

        // 첫 번째 파트: 전체 요약
        let mut part1 = String::new();
        part1.push_str(&format!("🛡️ **{}** 서버 보안 리포트\n", report.server_name));
        part1.push_str(&format!(
            "📅 검사 시간: {}\n\n",
            report.check_timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        ));

        let level_emoji = match report.security_level {
            SecurityLevel::Excellent => "🟢",
            SecurityLevel::Good => "🔵",
            SecurityLevel::Average => "🟡",
            SecurityLevel::Poor => "🟠",
            SecurityLevel::Critical => "🔴",
        };

        part1.push_str(&format!(
            "{} **전체 보안 점수: {}/100** ({:?})\n\n",
            level_emoji, report.overall_score, report.security_level
        ));

        // 카테고리별 점수
        part1.push_str("📊 **카테고리별 점수**\n");
        for category in &report.categories {
            let category_emoji = if category.score >= 80 {
                "✅"
            } else if category.score >= 60 {
                "⚠️"
            } else {
                "❌"
            };
            part1.push_str(&format!(
                "{} {}: {}/100\n",
                category_emoji, category.name, category.score
            ));
        }

        parts.push(part1);

        // 두 번째 파트: 치명적 이슈 및 권장사항
        if !report.critical_issues.is_empty() || !report.recommendations.is_empty() {
            let mut part2 = String::new();

            if !report.critical_issues.is_empty() {
                part2.push_str("🚨 **치명적 보안 이슈**\n");
                for (i, issue) in report.critical_issues.iter().enumerate() {
                    part2.push_str(&format!("{}. **{}**\n", i + 1, issue.title));
                    part2.push_str(&format!("   해결: {}\n\n", issue.solution));
                }
            }

            if !report.recommendations.is_empty() {
                part2.push_str("💡 **개선 권장사항**\n");
                for rec in &report.recommendations {
                    let priority_emoji = match rec.priority {
                        Priority::Immediate => "🔥",
                        Priority::High => "⚠️",
                        Priority::Medium => "📋",
                        Priority::Low => "💡",
                    };
                    part2.push_str(&format!("{} {}\n", priority_emoji, rec.action));
                }
            }

            parts.push(part2);
        }

        parts
    }
}

// =====================
// Helpers
// =====================
async fn log_embed(ctx: &Context, channel: Option<ChannelId>, title: &str, desc: &str) {
    if let Some(ch) = channel {
        let embed = CreateEmbed::new()
            .title(title)
            .description(desc)
            .timestamp(Utc::now());

        let message = CreateMessage::new().embed(embed);

        let _ = ch.send_message(&ctx.http, message).await;
    }
}

fn cached_guild<'a>(ctx: &'a Context, guild_id: GuildId) -> Option<GuildRef<'a>> {
    guild_id.to_guild_cached(&ctx.cache)
}

fn account_age_days(ctx: &Context, user_id: UserId) -> Option<i64> {
    let user = ctx.cache.user(user_id)?;
    let created = user.created_at();
    let created_dt: DateTime<Utc> = DateTime::from_timestamp(created.timestamp(), 0)?;
    let now = Utc::now();
    Some((now - created_dt).num_days())
}

async fn days_since_join(ctx: &Context, guild_id: GuildId, user_id: UserId) -> Option<i64> {
    let guild = cached_guild(ctx, guild_id)?;
    let member = guild.member(ctx, user_id).await.ok()?;
    let joined = member.joined_at?;
    let joined_dt: DateTime<Utc> = DateTime::from_timestamp(joined.timestamp(), 0)?;
    Some((Utc::now() - joined_dt).num_days())
}

// =====================
// Handler
// =====================
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let user = &new_member.user;
        let title = "🟢 Member Joined";
        let desc = format!("user: {} (<@{}>)", user.name, user.id.get());
        log_embed(&ctx, STATE.read().await.log_channel, title, &desc).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        // 기본 명령
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "🏓 Pong! 봇이 살아있어요!").await {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "!안전" {
            let embed = CreateEmbed::new()
                .title("🔒 보안 감사 봇")
                .description("상호 보안 감사 시스템이 준비되었습니다!")
                .field("기능", "• 실제 서버 보안 스캔\n• 보안 체크리스트\n• 상호 감사 시스템", false)
                .color(0x00ff00);
            let message = CreateMessage::new().embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
                println!("Error sending embed: {:?}", why);
            }
        }

        if msg.content == "!체크리스트" {
            let checklist = vec![
                "2단계 인증 활성화 확인",
                "브라우저 확장 프로그램 권한 검토",
                "소셜미디어 공개 범위 설정",
                "자동 업데이트 활성화 상태",
                "VPN 사용 여부",
                "비밀번호 관리자 사용",
                "공공 Wi-Fi 사용 주의",
                "개인정보 백업 상태",
            ];

            let mut checklist_text = String::new();
            for (i, item) in checklist.iter().enumerate() {
                checklist_text.push_str(&format!("{}. {}\n", i + 1, item));
            }

            let embed = CreateEmbed::new()
                .title("🛡️ 보안 체크리스트")
                .description(checklist_text)
                .footer(serenity::builder::CreateEmbedFooter::new("각 항목을 확인하고 보안을 강화하세요!"))
                .color(0x3498db);
            let message = CreateMessage::new().embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
                println!("Error sending checklist: {:?}", why);
            }
        }

        // 짝매칭
        if msg.content.starts_with("!짝매칭") {
            let parts: Vec<&str> = msg.content.split_whitespace().collect();
            if parts.len() == 2 {
                if let Some(mentioned_user) = msg.mentions.first() {
                    let mut state = STATE.write().await;
                    let pair_added = state.audit.add_pair(msg.author.id, mentioned_user.id);
                    let response = if pair_added {
                        format!("🤝 {}님과 {}님이 상호 보안 감사 짝이 되었습니다!",
                                msg.author.name, mentioned_user.name)
                    } else {
                        "이미 등록된 짝입니다!".to_string()
                    };
                    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                        println!("Error sending pair message: {:?}", why);
                    }
                }
            } else {
                let help_msg = "사용법: `!짝매칭 @사용자`\n예시: `!짝매칭 @친구`";
                if let Err(why) = msg.channel_id.say(&ctx.http, help_msg).await {
                    println!("Error sending help: {:?}", why);
                }
            }
        }

        if msg.content == "!내짝" {
            let state = STATE.read().await;
            let user_pairs: Vec<_> = state.audit.buddy_pairs.iter()
                .filter(|(a, b)| *a == msg.author.id || *b == msg.author.id)
                .collect();

            let response = if user_pairs.is_empty() {
                "아직 짝이 없습니다. `!짝매칭 @사용자`로 짝을 만드세요!".to_string()
            } else {
                let mut pairs_text = "👥 나의 보안 감사 짝들:\n".to_string();
                for (a, b) in user_pairs {
                    let partner_id = if *a == msg.author.id { *b } else { *a };
                    pairs_text.push_str(&format!("• <@{}>\n", partner_id.get()));
                }
                pairs_text
            };
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                println!("Error sending pairs: {:?}", why);
            }
        }

        // =====================
        // =====================
        // !스캔 (전체 보안 감사)
        // =====================
        if msg.content.starts_with("!스캔") || msg.content.starts_with("!서버스캔") {
            let guild_id = match msg.guild_id {
                Some(id) => id,
                None => {
                    let _ = msg.channel_id
                        .say(&ctx.http, "❌ 이 명령어는 서버에서만 사용 가능합니다.")
                        .await;
                    return;
                }
            };

            // ⚠️ 여기서는 절대 await 하지 말 것! (CacheRef 수명 때문에)
            // CacheRef를 소유 Guild로 복사(clone)한 값을 꺼내놓고, 참조를 여기서 끝낸다.
            let guild_owned_opt = {
                if let Some(g) = ctx.cache.guild(guild_id) {
                    Some(g.clone()) // Guild는 Clone 가능 (serenity 모델)
                } else {
                    None
                }
            };

            // 이제야 await 사용 (CacheRef는 이미 범위 밖으로 드롭됨)
            let guild_owned = match guild_owned_opt {
                Some(g) => g,
                None => {
                    let _ = msg.channel_id
                        .say(&ctx.http, "❌ 서버 정보를 가져올 수 없습니다.")
                        .await;
                    return;
                }
            };

            let _ = msg.channel_id
                .say(&ctx.http, "🔍 서버 보안 감사를 시작합니다... (약 10초 소요)")
                .await;

            // 보안 검사 수행
            let report = SecurityScanner::perform_security_audit(&ctx, &guild_owned).await;

            // 보고서 저장 (키: u64)
            {
                let mut state = STATE.write().await;
                state.security_reports
                    .entry(guild_id.get())
                    .or_insert_with(Vec::new)
                    .push(report.clone());
            }

            // 결과 전송
            let report_parts = SecurityScanner::format_security_report(&report);
            for part in report_parts {
                let _ = msg.channel_id.say(&ctx.http, part).await;
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            }
        }

        // =====================
        // !빠른스캔 (요약 점검)
        // =====================
        if msg.content == "!빠른스캔" {
            let guild_id = match msg.guild_id {
                Some(id) => id,
                None => {
                    let _ = msg.channel_id
                        .say(&ctx.http, "❌ 서버에서만 사용 가능합니다.")
                        .await;
                    return;
                }
            };

            // ⚠️ 이 블록 안에서는 절대 await 하지 말 것!
            // CacheRef에서 필요한 값만 소유 형태로 뽑아서 옵션에 담는다.
            let extracted = {
                if let Some(g) = ctx.cache.guild(guild_id) {
                    let total_members = g.members.len();
                    let bot_count = g.members.iter().filter(|(_, m)| m.user.bot).count();
                    let admin_roles = g.roles.values()
                        .filter(|role| role.permissions.administrator())
                        .count();

                    Some((
                        g.name.clone(),
                        g.verification_level,
                        g.mfa_level,
                        g.explicit_content_filter,
                        total_members,
                        bot_count,
                        admin_roles,
                    ))
                } else {
                    None
                }
            };

            // 여기서 await 사용 (CacheRef는 이미 범위 밖으로 드롭됨)
            let (guild_name, verification_level, mfa_level, filter_level, total_members, bot_count, admin_roles) =
                match extracted {
                    Some(t) => t,
                    None => {
                        let _ = msg.channel_id
                            .say(&ctx.http, "❌ 서버 정보를 가져올 수 없습니다.")
                            .await;
                        return;
                    }
                };

            let mut quick_report = String::new();
            quick_report.push_str(&format!("⚡ **{}** 빠른 보안 점검\n\n", guild_name));

            let verification_status = match verification_level {
                VerificationLevel::None => "❌ 없음 (위험)",
                VerificationLevel::Low => "⚠️ 낮음",
                VerificationLevel::Medium => "✅ 보통",
                VerificationLevel::High => "✅ 높음",
                VerificationLevel::Higher => "✅ 매우 높음",
                _ => "❓ 알 수 없음",
            };

            let mfa_status = match mfa_level {
                MfaLevel::None => "❌ 비활성화",
                MfaLevel::Elevated => "✅ 활성화",
                _ => "❓ 알 수 없음",
            };

            let filter_status = match filter_level {
                ExplicitContentFilter::None => "❌ 비활성화",
                ExplicitContentFilter::WithoutRole => "⚠️ 부분적",
                ExplicitContentFilter::All => "✅ 전체",
                _ => "❓ 알 수 없음",
            };

            quick_report.push_str(&format!("🔐 인증 레벨: {}\n", verification_status));
            quick_report.push_str(&format!("🛡️ 2단계 인증: {}\n", mfa_status));
            quick_report.push_str(&format!("🔒 콘텐츠 필터: {}\n", filter_status));

            let bot_ratio = if total_members > 0 { (bot_count * 100) / total_members } else { 0 };
            quick_report.push_str(&format!("\n👥 총 멤버: {}명\n", total_members));
            quick_report.push_str(&format!("🤖 봇: {}개 ({}%)\n", bot_count, bot_ratio));
            quick_report.push_str(&format!("⚡ 관리자 역할: {}개\n", admin_roles));

            let mut risk_count = 0;
            if matches!(verification_level, VerificationLevel::None) { risk_count += 1; }
            if matches!(mfa_level, MfaLevel::None) { risk_count += 1; }
            if bot_ratio > 30 { risk_count += 1; }
            if admin_roles > 5 { risk_count += 1; }

            let risk_level = match risk_count {
                0 => "🟢 낮음",
                1 => "🟡 보통",
                2 => "🟠 높음",
                _ => "🔴 매우 높음",
            };

            quick_report.push_str(&format!("\n📊 위험도: {}\n", risk_level));
            quick_report.push_str("\n💡 상세한 분석을 원하시면 `!스캔`을 사용하세요.");

            let _ = msg.channel_id.say(&ctx.http, quick_report).await;
        }

        // =====================
        // !스캔기록
        // =====================
        if msg.content == "!스캔기록" {
            let guild_id_u64 = match msg.guild_id {
                Some(id) => id.get(),
                None => {
                    let _ = msg.channel_id.say(&ctx.http, "❌ 이 명령어는 서버에서만 사용 가능합니다.").await;
                    return;
                }
            };

            let state = STATE.read().await;
            if let Some(reports) = state.security_reports.get(&guild_id_u64) {
                if reports.is_empty() {
                    let _ = msg.channel_id.say(&ctx.http, "📋 아직 보안 감사 기록이 없습니다. `!스캔`으로 첫 번째 감사를 시작하세요.").await;
                    return;
                }

                let mut history = "📊 **보안 감사 이력**\n\n".to_string();
                for (i, report) in reports.iter().rev().take(5).enumerate() {
                    let level_emoji = match report.security_level {
                        SecurityLevel::Excellent => "🟢",
                        SecurityLevel::Good => "🔵",
                        SecurityLevel::Average => "🟡",
                        SecurityLevel::Poor => "🟠",
                        SecurityLevel::Critical => "🔴",
                    };
                    history.push_str(&format!(
                        "{}. {} **{}점** ({:?})\n   📅 {}\n\n",
                        i + 1,
                        level_emoji,
                        report.overall_score,
                        report.security_level,
                        report.check_timestamp.format("%Y-%m-%d %H:%M")
                    ));
                }
                let _ = msg.channel_id.say(&ctx.http, history).await;
            } else {
                let _ = msg.channel_id.say(&ctx.http, "📋 아직 보안 감사 기록이 없습니다.").await;
            }
        }

        // =====================
        // 게임화/가이드
        // =====================
        if msg.content == "!서버점검" {
            let embed = CreateEmbed::new()
                .title("🔍 서버 보안 점검 가이드")
                .description("친구들과 함께 서버 보안을 점검해보세요!")
                .field("1단계: 스크린샷 준비",
                       "• 서버 설정 → 개요 페이지\n• 서버 설정 → 역할 → @everyone 권한\n• 서버 설정 → 감사 로그", false)
                .field("2단계: 점검 포인트",
                       "• @everyone 권한 (관리자/킥/밴 권한 있으면 위험!)\n• 봇 역할 권한 (최소 권한 원칙)\n• 채널별 권한 설정\n• 감사 로그 활성화 여부", false)
                .field("3단계: 점수 매기기",
                       "• 안전: 80-100점 🟢\n• 주의: 60-79점 🟡\n• 위험: 0-59점 🔴", false)
                .footer(serenity::builder::CreateEmbedFooter::new("이제 `!스캔` 명령어로 자동 분석도 가능합니다!"))
                .color(0xe74c3c);
            let message = CreateMessage::new().embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
                println!("Error sending server check: {:?}", why);
            }
        }

        if msg.content == "!챌린지" {
            let embed = CreateEmbed::new()
                .title("🏆 이번 주 보안 챌린지")
                .description("친구들과 경쟁하며 보안을 강화하세요!")
                .field("참여 방법",
                       "1. `!스캔` - 서버 보안 점수 확인\n2. `!빠른스캔` - 간단한 점검\n3. `!스캔기록` - 개선 추이 확인", false)
                .field("이번 주 미션",
                       "• 서버 보안 점수 80점 이상 달성\n• @everyone 권한 정리\n• 2단계 인증 활성화\n• 봇 권한 최소화", false)
                .field("보너스 점수",
                       "• 친구 도와주기 (+5점)\n• 보안 팁 공유 (+3점)\n• 정기적인 점검 (+10점)", false)
                .color(0xf39c12);
            let message = CreateMessage::new().embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
                println!("Error sending challenge: {:?}", why);
            }
        }

        if msg.content == "!실시간점검" {
            let embed = CreateEmbed::new()
                .title("🎥 실시간 보안 점검")
                .description("음성채팅에서 화면공유로 즉석 점검!")
                .field("준비물", "• 음성채팅 참가\n• 화면공유 준비\n• 점검할 설정 화면", false)
                .field("점검 순서",
                       "1️⃣ `!스캔` 으로 자동 분석 먼저\n2️⃣ 화면공유로 설정 보여주기\n3️⃣ 친구들과 함께 개선하기\n4️⃣ `!스캔`으로 점수 확인", false)
                .field("점검 중 할 일",
                       "• \"어? 저기 위험해!\" 🚨\n• \"그건 이렇게 고쳐!\" 💡\n• \"와 점수 올랐다!\" 🏆", false)
                .footer(serenity::builder::CreateEmbedFooter::new("이제 자동 스캔과 수동 점검을 함께 활용하세요!"))
                .color(0x9b59b6);
            let message = CreateMessage::new().embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
                println!("Error sending realtime check: {:?}", why);
            }
        }

        if msg.content == "!도움말" || msg.content == "!help" {
            let embed = CreateEmbed::new()
                .title("🛡️ 보안 감사 봇 명령어")
                .description("사용 가능한 모든 명령어들")
                .field("🔍 자동 스캔 명령어",
                       "`!스캔` - 전체 서버 보안 분석\n`!빠른스캔` - 간단한 보안 점검\n`!스캔기록` - 감사 기록 확인", false)
                .field("🤝 상호 감사 명령어",
                       "`!짝매칭 @사용자` - 감사 짝 만들기\n`!내짝` - 현재 짝 확인\n`!체크리스트` - 보안 체크리스트", false)
                .field("🎮 게임화 명령어",
                       "`!서버점검` - 수동 점검 가이드\n`!챌린지` - 주간 보안 챌린지\n`!실시간점검` - 화면공유 점검법", false)
                .field("ℹ️ 기타",
                       "`!ping` - 봇 상태 확인\n`!안전` - 봇 소개\n`!도움말` - 이 메뉴", false)
                .footer(serenity::builder::CreateEmbedFooter::new("자동 스캔과 상호 감사로 서버를 안전하게!"))
                .color(0x3498db);
            let message = CreateMessage::new().embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
                println!("Error sending help: {:?}", why);
            }
        }
    }

    async fn presence_update(&self, ctx: Context, new_data: Presence) {
        let user_id = new_data.user.id;
        if new_data.status == OnlineStatus::Online {
            // 캐시에서 이름 조회 (없으면 ID로 대체)
            let display = if let Some(u) = ctx.cache.user(user_id) {
                u.name.clone()
            } else {
                format!("<@{}>", user_id.get())
            };

            let msg = format!("🔔 {} 님이 온라인으로 전환했습니다!", display);
            let log_ch = STATE.read().await.log_channel;
            log_embed(&ctx, log_ch, "Presence Update", &msg).await;
        }
    }
}

// =====================
// main
// =====================
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    {
        let mut st = STATE.write().await;
        *st = AppState::from_env();
    }

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_PRESENCES
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
