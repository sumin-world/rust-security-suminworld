use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        guild::{Guild, Member, Role},
        permissions::Permissions,
        id::{ChannelId, RoleId, UserId},
    },
    prelude::*,
};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

// =============================================================================
// 서버 보안 점검 구조체들
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSecurityReport {
    pub server_name: String,
    pub check_timestamp: DateTime<Utc>,
    pub overall_score: u8, // 0-100
    pub security_level: SecurityLevel,
    pub categories: Vec<SecurityCategory>,
    pub critical_issues: Vec<SecurityIssue>,
    pub recommendations: Vec<SecurityRecommendation>,
    pub member_analysis: MemberAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Excellent,  // 90-100점 🟢
    Good,       // 70-89점  🔵  
    Average,    // 50-69점  🟡
    Poor,       // 30-49점  🟠
    Critical,   // 0-29점   🔴
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCategory {
    pub name: String,
    pub score: u8,
    pub weight: u8, // 전체 점수에서의 비중
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
    pub affected_items: Vec<String>,
    pub solution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRecommendation {
    pub category: String,
    pub action: String,
    pub priority: Priority,
    pub estimated_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Immediate, // 즉시
    High,      // 높음
    Medium,    // 보통
    Low,       // 낮음
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberAnalysis {
    pub total_members: usize,
    pub bot_count: usize,
    pub admin_count: usize,
    pub suspicious_members: Vec<SuspiciousMember>,
    pub new_members_last_week: usize,
    pub inactive_members: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousMember {
    pub user_id: u64,
    pub username: String,
    pub reasons: Vec<String>,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    High,
    Medium,
    Low,
}

// =============================================================================
// 메인 보안 점검 시스템
// =============================================================================

pub struct DiscordServerSecurityChecker {
    pub guild_id: u64,
    pub check_history: Vec<ServerSecurityReport>,
    pub whitelist: SecurityWhitelist,
}

#[derive(Debug, Clone)]
pub struct SecurityWhitelist {
    pub trusted_bots: Vec<u64>,
    pub verified_domains: Vec<String>,
    pub safe_invites: Vec<String>,
}

impl DiscordServerSecurityChecker {
    pub fn new(guild_id: u64) -> Self {
        Self {
            guild_id,
            check_history: Vec::new(),
            whitelist: SecurityWhitelist {
                trusted_bots: vec![
                    235088799074484224, // Carl-bot
                    155149108183695360, // Dyno
                    172002275412279296, // Tatsumaki
                    // 더 많은 신뢰할 수 있는 봇들...
                ],
                verified_domains: vec![
                    "discord.gg".to_string(),
                    "discord.com".to_string(),
                    "youtube.com".to_string(),
                    "github.com".to_string(),
                ],
                safe_invites: Vec::new(),
            },
        }
    }

    /// 서버 전체 보안 점검 실행
    pub async fn perform_security_audit(&mut self, ctx: &Context, guild: &Guild) -> ServerSecurityReport {
        let mut report = ServerSecurityReport {
            server_name: guild.name.clone(),
            check_timestamp: Utc::now(),
            overall_score: 0,
            security_level: SecurityLevel::Critical,
            categories: Vec::new(),
            critical_issues: Vec::new(),
            recommendations: Vec::new(),
            member_analysis: self.analyze_members(ctx, guild).await,
        };

        // 각 보안 카테고리 점검
        report.categories.push(self.check_permissions_security(guild).await);
        report.categories.push(self.check_channel_security(ctx, guild).await);
        report.categories.push(self.check_role_security(guild).await);
        report.categories.push(self.check_moderation_settings(guild).await);
        report.categories.push(self.check_bot_security(ctx, guild).await);
        report.categories.push(self.check_invite_security(ctx, guild).await);

        // 전체 점수 계산
        report.overall_score = self.calculate_overall_score(&report.categories);
        report.security_level = self.determine_security_level(report.overall_score);

        // 치명적 이슈 식별
        report.critical_issues = self.identify_critical_issues(&report.categories);

        // 개선 권장사항 생성
        report.recommendations = self.generate_recommendations(&report.categories, &report.critical_issues);

        // 히스토리에 저장
        self.check_history.push(report.clone());

        report
    }

    /// 권한 보안 점검
    async fn check_permissions_security(&self, guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        // 관리자 권한을 가진 역할 수 확인
        let admin_roles: Vec<_> = guild.roles.values()
            .filter(|role| role.permissions.administrator())
            .collect();

        if admin_roles.len() > 3 {
            checks.push(SecurityCheck {
                name: "관리자 역할 수".to_string(),
                status: CheckStatus::Warning,
                description: format!("{}개의 역할이 관리자 권한을 가지고 있습니다", admin_roles.len()),
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
                        description: format!("@everyone 역할이 위험한 '{}' 권한을 가지고 있습니다", perm_name),
                        impact: ImpactLevel::Critical,
                    });
                }
            }

            score = score.saturating_sub(dangerous_count * 20);
        }

        // 봇 역할의 과도한 권한 확인
        for role in guild.roles.values() {
            if role.managed && (role.permissions.administrator() || role.permissions.manage_guild()) {
                checks.push(SecurityCheck {
                    name: format!("봇 역할 '{}'", role.name),
                    status: CheckStatus::Warning,
                    description: "봇이 과도한 권한을 가지고 있습니다".to_string(),
                    impact: ImpactLevel::High,
                });
                score -= 10;
            }
        }

        SecurityCategory {
            name: "권한 보안".to_string(),
            score,
            weight: 30, // 30% 비중
            checks,
        }
    }

    /// 채널 보안 점검
    async fn check_channel_security(&self, ctx: &Context, guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        // 모든 채널 가져오기
        let channels = match guild.channels(ctx).await {
            Ok(channels) => channels,
            Err(_) => {
                checks.push(SecurityCheck {
                    name: "채널 접근".to_string(),
                    status: CheckStatus::Fail,
                    description: "채널 정보를 가져올 수 없습니다".to_string(),
                    impact: ImpactLevel::High,
                });
                return SecurityCategory {
                    name: "채널 보안".to_string(),
                    score: 0,
                    weight: 20,
                    checks,
                };
            }
        };

        let mut public_channels = 0;
        let mut private_channels = 0;

        for channel in channels.values() {
            // @everyone이 볼 수 있는 채널인지 확인
            let everyone_can_view = channel.permissions_for_role(ctx, guild.id).await
                .map(|perms| perms.view_channel())
                .unwrap_or(true);

            if everyone_can_view {
                public_channels += 1;
            } else {
                private_channels += 1;
            }
        }

        // 공개 채널 비율 확인
        let total_channels = public_channels + private_channels;
        if total_channels > 0 {
            let public_ratio = (public_channels * 100) / total_channels;
            
            if public_ratio > 80 {
                checks.push(SecurityCheck {
                    name: "공개 채널 비율".to_string(),
                    status: CheckStatus::Warning,
                    description: format!("{}%의 채널이 공개되어 있습니다", public_ratio),
                    impact: ImpactLevel::Medium,
                });
                score -= 15;
            } else {
                checks.push(SecurityCheck {
                    name: "공개 채널 비율".to_string(),
                    status: CheckStatus::Pass,
                    description: "적절한 채널 공개 설정입니다".to_string(),
                    impact: ImpactLevel::Low,
                });
            }
        }

        // 중요한 채널들의 보안 확인
        for channel in channels.values() {
            let channel_name = channel.name.to_lowercase();
            if channel_name.contains("admin") || channel_name.contains("mod") || channel_name.contains("staff") {
                let everyone_can_view = channel.permissions_for_role(ctx, guild.id).await
                    .map(|perms| perms.view_channel())
                    .unwrap_or(true);

                if everyone_can_view {
                    checks.push(SecurityCheck {
                        name: format!("중요 채널 '{}'", channel.name),
                        status: CheckStatus::Fail,
                        description: "관리자 채널이 공개되어 있습니다".to_string(),
                        impact: ImpactLevel::Critical,
                    });
                    score -= 25;
                }
            }
        }

        SecurityCategory {
            name: "채널 보안".to_string(),
            score,
            weight: 20,
            checks,
        }
    }

    /// 역할 보안 점검
    async fn check_role_security(&self, guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        // 역할 계층 구조 확인
        let mut admin_roles = Vec::new();
        let mut mod_roles = Vec::new();
        let mut suspicious_roles = Vec::new();

        for role in guild.roles.values() {
            if role.permissions.administrator() {
                admin_roles.push(role);
            } else if role.permissions.manage_messages() || role.permissions.kick_members() {
                mod_roles.push(role);
            }

            // 의심스러운 역할명 확인
            let suspicious_names = ["everyone", "nitro", "admin", "owner"];
            for suspicious in &suspicious_names {
                if role.name.to_lowercase().contains(suspicious) && role.name != "@everyone" {
                    suspicious_roles.push(role);
                }
            }
        }

        // 관리자 역할 분석
        if admin_roles.len() > 5 {
            checks.push(SecurityCheck {
                name: "관리자 역할 수".to_string(),
                status: CheckStatus::Warning,
                description: format!("{}개의 관리자 역할이 있습니다", admin_roles.len()),
                impact: ImpactLevel::Medium,
            });
            score -= 10;
        }

        // 의심스러운 역할명 확인
        if !suspicious_roles.is_empty() {
            for role in &suspicious_roles {
                checks.push(SecurityCheck {
                    name: format!("의심스러운 역할 '{}'", role.name),
                    status: CheckStatus::Warning,
                    description: "역할명이 의심스럽습니다".to_string(),
                    impact: ImpactLevel::Medium,
                });
            }
            score -= suspicious_roles.len() as u8 * 5;
        }

        // 색상이 없는 중요 역할 확인
        for role in &admin_roles {
            if role.colour.0 == 0 {
                checks.push(SecurityCheck {
                    name: format!("역할 '{}' 색상", role.name),
                    status: CheckStatus::Info,
                    description: "중요 역할에 색상이 없어 식별이 어려울 수 있습니다".to_string(),
                    impact: ImpactLevel::Low,
                });
            }
        }

        SecurityCategory {
            name: "역할 보안".to_string(),
            score,
            weight: 15,
            checks,
        }
    }

    /// 조정 설정 점검
    async fn check_moderation_settings(&self, guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        // 인증 레벨 확인
        match guild.verification_level {
            serenity::model::guild::VerificationLevel::None => {
                checks.push(SecurityCheck {
                    name: "인증 레벨".to_string(),
                    status: CheckStatus::Fail,
                    description: "인증 레벨이 '없음'으로 설정되어 있습니다".to_string(),
                    impact: ImpactLevel::High,
                });
                score -= 20;
            },
            serenity::model::guild::VerificationLevel::Low => {
                checks.push(SecurityCheck {
                    name: "인증 레벨".to_string(),
                    status: CheckStatus::Warning,
                    description: "인증 레벨이 '낮음'입니다".to_string(),
                    impact: ImpactLevel::Medium,
                });
                score -= 10;
            },
            _ => {
                checks.push(SecurityCheck {
                    name: "인증 레벨".to_string(),
                    status: CheckStatus::Pass,
                    description: "적절한 인증 레벨이 설정되어 있습니다".to_string(),
                    impact: ImpactLevel::Low,
                });
            }
        }

        // 명시적 콘텐츠 필터 확인
        match guild.explicit_content_filter {
            serenity::model::guild::ExplicitContentFilter::None => {
                checks.push(SecurityCheck {
                    name: "콘텐츠 필터".to_string(),
                    status: CheckStatus::Fail,
                    description: "명시적 콘텐츠 필터가 비활성화되어 있습니다".to_string(),
                    impact: ImpactLevel::High,
                });
                score -= 15;
            },
            serenity::model::guild::ExplicitContentFilter::MembersWithoutRoles => {
                checks.push(SecurityCheck {
                    name: "콘텐츠 필터".to_string(),
                    status: CheckStatus::Warning,
                    description: "일부 멤버만 콘텐츠 필터가 적용됩니다".to_string(),
                    impact: ImpactLevel::Medium,
                });
                score -= 5;
            },
            serenity::model::guild::ExplicitContentFilter::AllMembers => {
                checks.push(SecurityCheck {
                    name: "콘텐츠 필터".to_string(),
                    status: CheckStatus::Pass,
                    description: "모든 멤버에게 콘텐츠 필터가 적용됩니다".to_string(),
                    impact: ImpactLevel::Low,
                });
            },
        }

        // MFA 요구사항 확인
        if guild.mfa_level == serenity::model::guild::MfaLevel::None {
            checks.push(SecurityCheck {
                name: "2단계 인증".to_string(),
                status: CheckStatus::Fail,
                description: "관리자에게 2단계 인증이 요구되지 않습니다".to_string(),
                impact: ImpactLevel::Critical,
            });
            score -= 25;
        } else {
            checks.push(SecurityCheck {
                name: "2단계 인증".to_string(),
                status: CheckStatus::Pass,
                description: "관리자에게 2단계 인증이 요구됩니다".to_string(),
                impact: ImpactLevel::Low,
            });
        }

        SecurityCategory {
            name: "조정 설정".to_string(),
            score,
            weight: 25,
            checks,
        }
    }

    /// 봇 보안 점검
    async fn check_bot_security(&self, ctx: &Context, guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        let mut bot_count = 0;
        let mut untrusted_bots = Vec::new();
        let mut high_perm_bots = Vec::new();

        for member in &guild.members {
            if member.user.bot {
                bot_count += 1;

                // 신뢰할 수 없는 봇 확인
                if !self.whitelist.trusted_bots.contains(&member.user.id.0) {
                    untrusted_bots.push(member);
                }

                // 높은 권한을 가진 봇 확인
                let member_permissions = match member.permissions(ctx) {
                    Ok(perms) => perms,
                    Err(_) => continue,
                };

                if member_permissions.administrator() || member_permissions.manage_guild() {
                    high_perm_bots.push(member);
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
            }
        }

        // 신뢰할 수 없는 봇들
        if !untrusted_bots.is_empty() {
            checks.push(SecurityCheck {
                name: "미확인 봇".to_string(),
                status: CheckStatus::Warning,
                description: format!("{}개의 미확인 봇이 있습니다", untrusted_bots.len()),
                impact: ImpactLevel::Medium,
            });
            score -= untrusted_bots.len() as u8 * 5;
        }

        // 높은 권한을 가진 봇들
        if !high_perm_bots.is_empty() {
            checks.push(SecurityCheck {
                name: "고권한 봇".to_string(),
                status: CheckStatus::Warning,
                description: format!("{}개의 봇이 관리자 권한을 가지고 있습니다", high_perm_bots.len()),
                impact: ImpactLevel::High,
            });
            score -= high_perm_bots.len() as u8 * 10;
        }

        SecurityCategory {
            name: "봇 보안".to_string(),
            score,
            weight: 10,
            checks,
        }
    }

    /// 초대 링크 보안 점검  
    async fn check_invite_security(&self, ctx: &Context, guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        // 초대 링크 목록 가져오기
        match guild.invites(ctx).await {
            Ok(invites) => {
                let mut permanent_invites = 0;
                let mut unlimited_invites = 0;

                for invite in invites {
                    if invite.max_age == 0 {
                        permanent_invites += 1;
                    }
                    if invite.max_uses == 0 {
                        unlimited_invites += 1;
                    }
                }

                if permanent_invites > 2 {
                    checks.push(SecurityCheck {
                        name: "영구 초대링크".to_string(),
                        status: CheckStatus::Warning,
                        description: format!("{}개의 영구 초대링크가 있습니다", permanent_invites),
                        impact: ImpactLevel::Medium,
                    });
                    score -= 10;
                }

                if unlimited_invites > 3 {
                    checks.push(SecurityCheck {
                        name: "무제한 초대링크".to_string(),
                        status: CheckStatus::Warning,
                        description: format!("{}개의 무제한 사용 초대링크가 있습니다", unlimited_invites),
                        impact: ImpactLevel::Medium,
                    });
                    score -= 10;
                }
            },
            Err(_) => {
                checks.push(SecurityCheck {
                    name: "초대링크 접근".to_string(),
                    status: CheckStatus::Fail,
                    description: "초대링크 정보를 가져올 수 없습니다".to_string(),
                    impact: ImpactLevel::Medium,
                });
                score -= 20;
            }
        }

        SecurityCategory {
            name: "초대 보안".to_string(),
            score,
            weight: 10,
            checks,
        }
    }

    /// 멤버 분석
    async fn analyze_members(&self, ctx: &Context, guild: &Guild) -> MemberAnalysis {
        let total_members = guild.members.len();
        let mut bot_count = 0;
        let mut admin_count = 0;
        let mut suspicious_members = Vec::new();
        let mut new_members_last_week = 0;
        let mut inactive_members = 0;

        let week_ago = Utc::now() - Duration::weeks(1);

        for member in &guild.members {
            // 봇 카운트
            if member.user.bot {
                bot_count += 1;
                continue;
            }

            // 관리자 카운트
            if let Ok(permissions) = member.permissions(ctx) {
                if permissions.administrator() {
                    admin_count += 1;
                }
            }

            // 새 멤버 (최근 1주일)
            if member.joined_at.map(|join_time| join_time.timestamp() as i64) 
                .unwrap_or(0) > week_ago.timestamp() {
                new_members_last_week += 1;
            }

            // 의심스러운 멤버 패턴 확인
            let mut suspicion_reasons = Vec::new();

            // 계정 나이가 너무 새로움 (1주일 미만)
            if let Some(created_at) = member.user.created_at().checked_sub_signed(Duration::weeks(1)) {
                if created_at > week_ago {
                    suspicion_reasons.push("새로 생성된 계정".to_string());
                }
            }

            // 의심스러운 사용자명 패턴
            let username = &member.user.name;
            if username.chars().filter(|c| c.is_numeric()).count() > username.len() / 2 {
                suspicion_reasons.push("숫자가 많은 사용자명".to_string());
            }

            // 기본 아바타 사용
            if member.user.avatar.is_none() {
                suspicion_reasons.push("기본 아바타 사용".to_string());
            }

            if !suspicion_reasons.is_empty() {
                let risk_level = if suspicion_reasons.len() >= 3 {
                    RiskLevel::High
                } else if suspicion_reasons.len() >= 2 {
                    RiskLevel::Medium
                } else {
                    RiskLevel::Low
                };

                suspicious_members.push(SuspiciousMember {
                    user_id: member.user.id.0,
                    username: username.clone(),
                    reasons: suspicion_reasons,
                    risk_level,
                });
            }
        }

        MemberAnalysis {
            total_members,
            bot_count,
            admin_count,
            suspicious_members,
            new_members_last_week,
            inactive_members,
        }
    }

    /// 전체 점수 계산
    fn calculate_overall_score(&self, categories: &[SecurityCategory]) -> u8 {
        let total_weight: u8 = categories.iter().map(|c| c.weight).sum();
        if total_weight == 0 {
            return 0;
        }

        let weighted_score: u32 = categories.iter()
            .map(|c| c.score as u32 * c.weight as u32)
            .sum();

        (weighted_score / total_weight as u32) as u8
    }

    /// 보안 레벨 결정
    fn determine_security_level(&self, score: u8) -> SecurityLevel {
        match score {
            90..=100 => SecurityLevel::Excellent,
            70..=89 => SecurityLevel::Good,
            50..=69 => SecurityLevel::Average,
            30..=49 => SecurityLevel::Poor,
            _ => SecurityLevel::Critical,
        }
    }

    /// 치명적 이슈 식별
    fn identify_critical_issues(&self, categories: &[SecurityCategory]) -> Vec<SecurityIssue> {
        let mut critical_issues = Vec::new();

        for category in categories {
            for check in &category.checks {
                if matches!(check.impact, ImpactLevel::Critical) && matches!(check.status, CheckStatus::Fail) {
                    critical_issues.push(SecurityIssue {
                        title: check.name.clone(),
                        description: check.description.clone(),
                        severity: check.impact.clone(),
                        affected_items: vec![category.name.clone()],
                        solution: self.get_solution_for_check(&check.name),
                    });
                }
            }
        }

        critical_issues
    }

    /// 권장사항 생성
    fn generate_recommendations(&self, categories: &[SecurityCategory], critical_issues: &[SecurityIssue]) -> Vec<SecurityRecommendation> {
        let mut recommendations = Vec::new();

        // 치명적 이슈 기반 권장사항
        for issue in critical_issues {
            recommendations.push(SecurityRecommendation {
                category: issue.affected_items.first().unwrap_or(&"일반".to_string()).clone(),
                action: issue.solution.clone(),
                priority: Priority::Immediate,
                estimated_impact: "보안 위험 크게 감소".to_string(),
            });
        }

        // 카테고리별 권장사항
        for category in categories {
            if category.score < 70 {
                let recommendation = match category.name.as_str() {
                    "권한 보안" => SecurityRecommendation {
                        category: category.name.clone(),
                        action: "관리자 권한을 가진 역할 수를 줄이고, @everyone 권한을 검토하세요".to_string(),
                        priority: Priority::High,
                        estimated_impact: "권한 남용 위험 감소".to_string(),
                    },
                    "채널 보안" => SecurityRecommendation {
                        category: category.name.clone(),
                        action: "중요한 채널의 접근 권한을 제한하고 채널별 권한을 세분화하세요".to_string(),
                        priority: Priority::Medium,
                        estimated_impact: "정보 유출 위험 감소".to_string(),
                    },
                    "조정 설정" => SecurityRecommendation {
                        category: category.name.clone(),
                        action: "인증 레벨을 높이고 2단계 인증을 활성화하세요".to_string(),
                        priority: Priority::High,
                        estimated_impact: "스팸 및 악성 사용자 차단".to_string(),
                    },
                    "봇 보안" => SecurityRecommendation {
                        category: category.name.clone(),
                        action: "불필요한 봇을 제거하고 봇 권한을 최소화하세요".to_string(),
                        priority: Priority::Medium,
                        estimated_impact: "봇 관련 보안 위험 감소".to_string(),
                    },
                    _ => SecurityRecommendation {
                        category: category.name.clone(),
                        action: format!("{} 관련 설정을 검토하고 개선하세요", category.name),
                        priority: Priority::Medium,
                        estimated_impact: "전반적인 보안 향상".to_string(),
                    },
                };
                recommendations.push(recommendation);
            }
        }

        recommendations
    }

    /// 특정 검사에 대한 해결책 제공
    fn get_solution_for_check(&self, check_name: &str) -> String {
        match check_name {
            name if name.contains("@everyone") => {
                "서버 설정 → 역할 → @everyone → 위험한 권한들을 비활성화하세요".to_string()
            },
            name if name.contains("관리자 역할") => {
                "불필요한 관리자 권한을 제거하고 필요한 경우에만 부여하세요".to_string()
            },
            name if name.contains("인증 레벨") => {
                "서버 설정 → 조정 → 인증 레벨을 '중간' 이상으로 설정하세요".to_string()
            },
            name if name.contains("2단계 인증") => {
                "서버 설정 → 조정 → 관리 작업에 2단계 인증 요구를 활성화하세요".to_string()
            },
            name if name.contains("콘텐츠 필터") => {
                "서버 설정 → 조정 → 명시적 콘텐츠 필터를 '모든 멤버'로 설정하세요".to_string()
            },
            _ => "해당 설정을 검토하고 보안 모범 사례에 따라 수정하세요".to_string(),
        }
    }

    /// 보고서를 사용자 친화적 형태로 포맷팅
    pub fn format_security_report(&self, report: &ServerSecurityReport) -> String {
        let mut output = String::new();

        // 헤더
        output.push_str(&format!("🛡️ **{}** 서버 보안 리포트\n", report.server_name));
        output.push_str(&format!("📅 검사 시간: {}\n\n", report.check_timestamp.format("%Y-%m-%d %H:%M:%S UTC")));

        // 전체 점수와 등급
        let level_emoji = match report.security_level {
            SecurityLevel::Excellent => "🟢",
            SecurityLevel::Good => "🔵",
            SecurityLevel::Average => "🟡",
            SecurityLevel::Poor => "🟠",
            SecurityLevel::Critical => "🔴",
        };

        output.push_str(&format!("{} **전체 보안 점수: {}/100** ({:?})\n\n", 
            level_emoji, report.overall_score, report.security_level));

        // 멤버 분석 요약
        output.push_str("👥 **멤버 분석**\n");
        output.push_str(&format!("├ 총 멤버: {}명\n", report.member_analysis.total_members));
        output.push_str(&format!("├ 봇: {}개\n", report.member_analysis.bot_count));
        output.push_str(&format!("├ 관리자: {}명\n", report.member_analysis.admin_count));
        output.push_str(&format!("├ 신규 멤버 (1주일): {}명\n", report.member_analysis.new_members_last_week));
        output.push_str(&format!("└ 의심스러운 멤버: {}명\n\n", report.member_analysis.suspicious_members.len()));

        // 카테고리별 점수
        output.push_str("📊 **카테고리별 점수**\n");
        for category in &report.categories {
            let category_emoji = if category.score >= 80 { "✅" } else if category.score >= 60 { "⚠️" } else { "❌" };
            output.push_str(&format!("{} {}: {}/100\n", category_emoji, category.name, category.score));
        }
        output.push_str("\n");

        // 치명적 이슈
        if !report.critical_issues.is_empty() {
            output.push_str("🚨 **치명적 보안 이슈**\n");
            for (i, issue) in report.critical_issues.iter().enumerate() {
                output.push_str(&format!("{}. **{}**\n", i + 1, issue.title));
                output.push_str(&format!("   문제: {}\n", issue.description));
                output.push_str(&format!("   해결: {}\n\n", issue.solution));
            }
        }

        // 권장사항
        if !report.recommendations.is_empty() {
            output.push_str("💡 **개선 권장사항**\n");
            let mut immediate = Vec::new();
            let mut high = Vec::new();
            let mut medium = Vec::new();
            let mut low = Vec::new();

            for rec in &report.recommendations {
                match rec.priority {
                    Priority::Immediate => immediate.push(rec),
                    Priority::High => high.push(rec),
                    Priority::Medium => medium.push(rec),
                    Priority::Low => low.push(rec),
                }
            }

            if !immediate.is_empty() {
                output.push_str("🔥 **즉시 조치 필요:**\n");
                for rec in immediate {
                    output.push_str(&format!("• {}\n", rec.action));
                }
                output.push_str("\n");
            }

            if !high.is_empty() {
                output.push_str("⚠️ **높은 우선순위:**\n");
                for rec in high {
                    output.push_str(&format!("• {}\n", rec.action));
                }
                output.push_str("\n");
            }

            if !medium.is_empty() {
                output.push_str("📋 **보통 우선순위:**\n");
                for rec in medium {
                    output.push_str(&format!("• {}\n", rec.action));
                }
                output.push_str("\n");
            }
        }

        // 상세 정보
        output.push_str("🔍 **상세 분석 결과**\n");
        for category in &report.categories {
            output.push_str(&format!("\n**{}** ({}/100)\n", category.name, category.score));
            for check in &category.checks {
                let status_emoji = match check.status {
                    CheckStatus::Pass => "✅",
                    CheckStatus::Fail => "❌",
                    CheckStatus::Warning => "⚠️",
                    CheckStatus::Info => "ℹ️",
                };
                output.push_str(&format!("{} {}: {}\n", status_emoji, check.name, check.description));
            }
        }

        // 의심스러운 멤버 상세 정보 (높은 위험도만)
        let high_risk_members: Vec<_> = report.member_analysis.suspicious_members.iter()
            .filter(|m| matches!(m.risk_level, RiskLevel::High))
            .collect();

        if !high_risk_members.is_empty() {
            output.push_str("\n⚠️ **높은 위험도 멤버**\n");
            for member in high_risk_members {
                output.push_str(&format!("• {} (ID: {})\n", member.username, member.user_id));
                output.push_str(&format!("  이유: {}\n", member.reasons.join(", ")));
            }
        }

        output.push_str("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        output.push_str("💡 정기적인 보안 점검을 통해 서버를 안전하게 유지하세요!\n");

        output
    }
}

// =============================================================================
// Discord 봇 핸들러
// =============================================================================

struct SecurityAuditBot {
    checkers: std::sync::Mutex<std::collections::HashMap<u64, DiscordServerSecurityChecker>>,
}

impl SecurityAuditBot {
    fn new() -> Self {
        Self {
            checkers: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    async fn handle_security_command(&self, ctx: &Context, msg: &Message, args: Vec<&str>) -> String {
        let guild_id = match msg.guild_id {
            Some(id) => id.0,
            None => return "❌ 이 명령어는 서버에서만 사용 가능합니다.".to_string(),
        };

        match args.get(0) {
            Some(&"scan") | Some(&"audit") => {
                self.perform_full_audit(ctx, msg, guild_id).await
            },
            Some(&"quick") => {
                self.perform_quick_check(ctx, msg, guild_id).await
            },
            Some(&"history") => {
                self.show_audit_history(guild_id).await
            },
            Some(&"help") => {
                self.show_help().await
            },
            _ => "❓ 사용법: `!security [scan|quick|history|help]`".to_string(),
        }
    }

    async fn perform_full_audit(&self, ctx: &Context, msg: &Message, guild_id: u64) -> String {
        let guild = match msg.guild(ctx) {
            Some(guild) => guild,
            None => return "❌ 서버 정보를 가져올 수 없습니다.".to_string(),
        };

        // 권한 확인
        if let Ok(permissions) = msg.member.as_ref().unwrap().permissions(ctx) {
            if !permissions.manage_guild() {
                return "❌ 이 명령어를 사용하려면 '서버 관리' 권한이 필요합니다.".to_string();
            }
        }

        let _ = msg.channel_id.say(ctx, "🔍 서버 보안 감사를 시작합니다... (약 30초 소요)").await;

        // 보안 검사 수행
        let mut checkers = self.checkers.lock().unwrap();
        let checker = checkers.entry(guild_id).or_insert_with(|| DiscordServerSecurityChecker::new(guild_id));
        
        let report = checker.perform_security_audit(ctx, &guild).await;
        
        // 결과 포맷팅 및 전송
        let formatted_report = checker.format_security_report(&report);
        
        // Discord 메시지 길이 제한 고려
        if formatted_report.len() > 2000 {
            // 긴 보고서는 여러 메시지로 분할
            let parts = self.split_long_message(&formatted_report, 1900);
            for part in parts {
                let _ = msg.channel_id.say(ctx, part).await;
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        } else {
            let _ = msg.channel_id.say(ctx, formatted_report).await;
        }

        "✅ 보안 감사가 완료되었습니다.".to_string()
    }

    async fn perform_quick_check(&self, ctx: &Context, msg: &Message, guild_id: u64) -> String {
        let guild = match msg.guild(ctx) {
            Some(guild) => guild,
            None => return "❌ 서버 정보를 가져올 수 없습니다.".to_string(),
        };

        let mut quick_report = String::new();
        quick_report.push_str(&format!("⚡ **{}** 빠른 보안 점검\n\n", guild.name));

        // 기본 설정 확인
        let verification_status = match guild.verification_level {
            serenity::model::guild::VerificationLevel::None => "❌ 없음 (위험)",
            serenity::model::guild::VerificationLevel::Low => "⚠️ 낮음",
            serenity::model::guild::VerificationLevel::Medium => "✅ 보통",
            serenity::model::guild::VerificationLevel::High => "✅ 높음",
            serenity::model::guild::VerificationLevel::VeryHigh => "✅ 매우 높음",
        };

        let mfa_status = match guild.mfa_level {
            serenity::model::guild::MfaLevel::None => "❌ 비활성화",
            serenity::model::guild::MfaLevel::Elevated => "✅ 활성화",
        };

        let filter_status = match guild.explicit_content_filter {
            serenity::model::guild::ExplicitContentFilter::None => "❌ 비활성화",
            serenity::model::guild::ExplicitContentFilter::MembersWithoutRoles => "⚠️ 부분적",
            serenity::model::guild::ExplicitContentFilter::AllMembers => "✅ 전체",
        };

        quick_report.push_str(&format!("🔐 인증 레벨: {}\n", verification_status));
        quick_report.push_str(&format!("🛡️ 2단계 인증: {}\n", mfa_status));
        quick_report.push_str(&format!("🔍 콘텐츠 필터: {}\n", filter_status));

        // 멤버 통계
        let total_members = guild.members.len();
        let bot_count = guild.members.iter().filter(|m| m.user.bot).count();
        let bot_ratio = if total_members > 0 { (bot_count * 100) / total_members } else { 0 };

        quick_report.push_str(&format!("\n👥 총 멤버: {}명\n", total_members));
        quick_report.push_str(&format!("🤖 봇: {}개 ({}%)\n", bot_count, bot_ratio));

        // 관리자 권한 확인
        let admin_roles = guild.roles.values()
            .filter(|role| role.permissions.administrator())
            .count();
        
        quick_report.push_str(&format!("⚡ 관리자 역할: {}개\n", admin_roles));

        // 간단한 위험도 평가
        let mut risk_count = 0;
        if matches!(guild.verification_level, serenity::model::guild::VerificationLevel::None) { risk_count += 1; }
        if matches!(guild.mfa_level, serenity::model::guild::MfaLevel::None) { risk_count += 1; }
        if bot_ratio > 30 { risk_count += 1; }
        if admin_roles > 5 { risk_count += 1; }

        let risk_level = match risk_count {
            0 => "🟢 낮음",
            1 => "🟡 보통", 
            2 => "🟠 높음",
            _ => "🔴 매우 높음",
        };

        quick_report.push_str(&format!("\n📊 위험도: {}\n", risk_level));
        quick_report.push_str("\n💡 상세한 분석을 원하시면 `!security scan`을 사용하세요.");

        quick_report
    }

    async fn show_audit_history(&self, guild_id: u64) -> String {
        let checkers = self.checkers.lock().unwrap();
        
        if let Some(checker) = checkers.get(&guild_id) {
            if checker.check_history.is_empty() {
                return "📝 아직 보안 감사 기록이 없습니다. `!security scan`으로 첫 번째 감사를 시작하세요.".to_string();
            }

            let mut history = "📊 **보안 감사 이력**\n\n".to_string();
            
            for (i, report) in checker.check_history.iter().rev().take(5).enumerate() {
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

            history
        } else {
            "📝 아직 보안 감사 기록이 없습니다.".to_string()
        }
    }

    async fn show_help(&self) -> String {
        "🛡️ **서버 보안 점검 도구 도움말**\n\n\
        **명령어:**\n\
        `!security scan` - 전체 보안 감사 실행\n\
        `!security quick` - 빠른 보안 점검\n\
        `!security history` - 감사 이력 확인\n\
        `!security help` - 이 도움말\n\n\
        **권한 요구사항:**\n\
        • 전체 감사: 서버 관리 권한 필요\n\
        • 빠른 점검: 모든 사용자 가능\n\n\
        **점검 항목:**\n\
        • 권한 설정 및 역할 보안\n\
        • 채널 접근 권한\n\
        • 조정 설정 (인증, 필터 등)\n\
        • 봇 보안 상태\n\
        • 멤버 분석 및 의심 계정 탐지\n\
        • 초대 링크 보안\n\n\
        💡 **팁:** 정기적인 보안 점검으로 서버를 안전하게 유지하세요!".to_string()
    }

    fn split_long_message(&self, message: &str, max_length: usize) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current_part = String::new();

        for line in message.lines() {
            if current_part.len() + line.len() + 1 > max_length {
                if !current_part.is_empty() {
                    parts.push(current_part.clone());
                    current_part.clear();
                }
            }
            
            if !current_part.is_empty() {
                current_part.push('\n');
            }
            current_part.push_str(line);
        }

        if !current_part.is_empty() {
            parts.push(current_part);
        }

        parts
    }
}

// =============================================================================
// Discord 봇 이벤트 핸들러
// =============================================================================

struct Handler {
    security_bot: SecurityAuditBot,
}

impl Handler {
    fn new() -> Self {
        Self {
            security_bot: SecurityAuditBot::new(),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let content = msg.content.trim();
        
        if content.starts_with("!security") {
            let args: Vec<&str> = content.split_whitespace().skip(1).collect();
            let response = self.security_bot.handle_security_command(&ctx, &msg, args).await;
            
            if !response.is_empty() && !response.contains("완료되었습니다") {
                let _ = msg.channel_id.say(&ctx.http, response).await;
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("🛡️ {} 서버 보안 점검 봇이 준비되었습니다!", ready.user.name);
        println!("📋 사용법: !security help");
    }
}

// =============================================================================
// 메인 함수
// =============================================================================

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN 환경변수를 설정해주세요");

    let intents = GatewayIntents::GUILD_MESSAGES 
        | GatewayIntents::DIRECT_MESSAGES 
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler::new())
        .await
        .expect("클라이언트 생성 실패");

    println!("🚀 Discord 서버 보안 점검 봇 시작 중...");
    
    if let Err(why) = client.start().await {
        println!("클라이언트 오류: {:?}", why);
    }
}