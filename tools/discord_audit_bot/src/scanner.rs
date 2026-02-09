use crate::models::*;
use serenity::model::guild::{ExplicitContentFilter, Guild, MfaLevel, VerificationLevel};
use serenity::prelude::*;

pub struct SecurityScanner;

impl SecurityScanner {
    pub async fn perform_security_audit(ctx: &Context, guild: &Guild) -> ServerSecurityReport {
        let mut report = ServerSecurityReport {
            server_name: guild.name.clone(),
            check_timestamp: chrono::Utc::now(),
            overall_score: 0,
            security_level: SecurityLevel::Critical,
            categories: Vec::new(),
            critical_issues: Vec::new(),
            recommendations: Vec::new(),
        };

        report.categories.push(Self::check_permissions(guild));
        report.categories.push(Self::check_moderation(guild));
        report.categories.push(Self::check_roles(guild));
        report.categories.push(Self::check_bots(ctx, guild));

        report.overall_score = Self::calculate_overall_score(&report.categories);
        report.security_level = Self::determine_level(report.overall_score);
        report.critical_issues = Self::identify_critical_issues(&report.categories);
        report.recommendations = Self::generate_recommendations(&report.categories);

        report
    }

    // ── Category checks ────────────────────────────────────────────

    fn check_permissions(guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

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

            let mut dangerous_count: u8 = 0;
            for (has_perm, perm_name) in dangerous_perms {
                if has_perm {
                    dangerous_count += 1;
                    checks.push(SecurityCheck {
                        name: format!("@everyone {perm_name}"),
                        status: CheckStatus::Fail,
                        description: format!(
                            "@everyone 역할이 위험한 '{perm_name}' 권한을 가지고 있습니다"
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

        let admin_roles_count = guild
            .roles
            .values()
            .filter(|r| r.permissions.administrator())
            .count();

        if admin_roles_count > 3 {
            checks.push(SecurityCheck {
                name: "관리자 역할 수".to_string(),
                status: CheckStatus::Warning,
                description: format!(
                    "{admin_roles_count}개의 역할이 관리자 권한을 가지고 있습니다"
                ),
                impact: ImpactLevel::Medium,
            });
            score = score.saturating_sub(15);
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

    fn check_moderation(guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        match guild.verification_level {
            VerificationLevel::None => {
                checks.push(SecurityCheck {
                    name: "인증 레벨".into(),
                    status: CheckStatus::Fail,
                    description: "인증 레벨이 '없음'으로 설정되어 있습니다".into(),
                    impact: ImpactLevel::High,
                });
                score -= 25;
            }
            VerificationLevel::Low => {
                checks.push(SecurityCheck {
                    name: "인증 레벨".into(),
                    status: CheckStatus::Warning,
                    description: "인증 레벨이 '낮음'입니다".into(),
                    impact: ImpactLevel::Medium,
                });
                score -= 10;
            }
            _ => {
                checks.push(SecurityCheck {
                    name: "인증 레벨".into(),
                    status: CheckStatus::Pass,
                    description: "적절한 인증 레벨이 설정되어 있습니다".into(),
                    impact: ImpactLevel::Low,
                });
            }
        }

        if guild.mfa_level == MfaLevel::None {
            checks.push(SecurityCheck {
                name: "2단계 인증".into(),
                status: CheckStatus::Fail,
                description: "관리자에게 2단계 인증이 요구되지 않습니다".into(),
                impact: ImpactLevel::Critical,
            });
            score -= 30;
        } else {
            checks.push(SecurityCheck {
                name: "2단계 인증".into(),
                status: CheckStatus::Pass,
                description: "관리자에게 2단계 인증이 요구됩니다".into(),
                impact: ImpactLevel::Low,
            });
        }

        match guild.explicit_content_filter {
            ExplicitContentFilter::None => {
                checks.push(SecurityCheck {
                    name: "콘텐츠 필터".into(),
                    status: CheckStatus::Fail,
                    description: "명시적 콘텐츠 필터가 비활성화되어 있습니다".into(),
                    impact: ImpactLevel::High,
                });
                score -= 20;
            }
            ExplicitContentFilter::WithoutRole => {
                checks.push(SecurityCheck {
                    name: "콘텐츠 필터".into(),
                    status: CheckStatus::Warning,
                    description: "일부 멤버만 콘텐츠 필터가 적용됩니다".into(),
                    impact: ImpactLevel::Medium,
                });
                score -= 10;
            }
            ExplicitContentFilter::All => {
                checks.push(SecurityCheck {
                    name: "콘텐츠 필터".into(),
                    status: CheckStatus::Pass,
                    description: "모든 멤버에게 콘텐츠 필터가 적용됩니다".into(),
                    impact: ImpactLevel::Low,
                });
            }
            _ => {
                checks.push(SecurityCheck {
                    name: "콘텐츠 필터".into(),
                    status: CheckStatus::Info,
                    description: "콘텐츠 필터 설정을 확인할 수 없습니다".into(),
                    impact: ImpactLevel::Low,
                });
            }
        }

        SecurityCategory {
            name: "조정 설정".into(),
            score,
            weight: 30,
            checks,
        }
    }

    fn check_roles(guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        let suspicious_names = ["everyone", "nitro", "admin", "owner"];
        let mut suspicious_count: u8 = 0;

        for role in guild.roles.values() {
            let lower = role.name.to_lowercase();
            if role.name != "@everyone" && suspicious_names.iter().any(|s| lower.contains(s)) {
                suspicious_count += 1;
                checks.push(SecurityCheck {
                    name: format!("의심스러운 역할 '{}'", role.name),
                    status: CheckStatus::Warning,
                    description: "역할명이 의심스럽습니다".into(),
                    impact: ImpactLevel::Medium,
                });
            }
        }

        if suspicious_count == 0 {
            checks.push(SecurityCheck {
                name: "역할명 검사".into(),
                status: CheckStatus::Pass,
                description: "의심스러운 역할명이 발견되지 않았습니다".into(),
                impact: ImpactLevel::Low,
            });
        } else {
            score = score.saturating_sub(suspicious_count * 10);
        }

        SecurityCategory {
            name: "역할 보안".into(),
            score,
            weight: 20,
            checks,
        }
    }

    fn check_bots(_ctx: &Context, guild: &Guild) -> SecurityCategory {
        let mut checks = Vec::new();
        let mut score = 100u8;

        let mut bot_count: usize = 0;
        let mut high_perm_bots: u8 = 0;

        for member in guild.members.values() {
            if member.user.bot {
                bot_count += 1;
                let has_admin = member.roles.iter().any(|role_id| {
                    guild.roles.get(role_id).is_some_and(|r| {
                        r.permissions.administrator() || r.permissions.manage_guild()
                    })
                });
                if has_admin {
                    high_perm_bots += 1;
                }
            }
        }

        let total_members = guild.members.len();
        if total_members > 0 {
            let bot_ratio = (bot_count * 100) / total_members;
            if bot_ratio > 30 {
                checks.push(SecurityCheck {
                    name: "봇 비율".into(),
                    status: CheckStatus::Warning,
                    description: format!("서버의 {bot_ratio}%가 봇입니다"),
                    impact: ImpactLevel::Medium,
                });
                score -= 15;
            } else {
                checks.push(SecurityCheck {
                    name: "봇 비율".into(),
                    status: CheckStatus::Pass,
                    description: format!("적절한 봇 비율입니다 ({bot_ratio}%)"),
                    impact: ImpactLevel::Low,
                });
            }
        }

        if high_perm_bots > 0 {
            checks.push(SecurityCheck {
                name: "고권한 봇".into(),
                status: CheckStatus::Warning,
                description: format!("{high_perm_bots}개의 봇이 관리자 권한을 가지고 있습니다"),
                impact: ImpactLevel::High,
            });
            score = score.saturating_sub(high_perm_bots * 15);
        } else if bot_count > 0 {
            checks.push(SecurityCheck {
                name: "봇 권한".into(),
                status: CheckStatus::Pass,
                description: "봇들이 적절한 권한을 가지고 있습니다".into(),
                impact: ImpactLevel::Low,
            });
        }

        SecurityCategory {
            name: "봇 보안".into(),
            score,
            weight: 15,
            checks,
        }
    }

    // ── Scoring helpers ────────────────────────────────────────────

    fn calculate_overall_score(categories: &[SecurityCategory]) -> u8 {
        let total_weight: u32 = categories.iter().map(|c| c.weight as u32).sum();
        if total_weight == 0 {
            return 0;
        }
        let weighted: u32 = categories
            .iter()
            .map(|c| c.score as u32 * c.weight as u32)
            .sum();
        (weighted / total_weight) as u8
    }

    fn determine_level(score: u8) -> SecurityLevel {
        match score {
            90..=100 => SecurityLevel::Excellent,
            70..=89 => SecurityLevel::Good,
            50..=69 => SecurityLevel::Average,
            30..=49 => SecurityLevel::Poor,
            _ => SecurityLevel::Critical,
        }
    }

    fn identify_critical_issues(categories: &[SecurityCategory]) -> Vec<SecurityIssue> {
        categories
            .iter()
            .flat_map(|cat| &cat.checks)
            .filter(|chk| {
                matches!(chk.impact, ImpactLevel::Critical)
                    && matches!(chk.status, CheckStatus::Fail)
            })
            .map(|chk| SecurityIssue {
                title: chk.name.clone(),
                description: chk.description.clone(),
                severity: chk.impact.clone(),
                solution: Self::solution_for(&chk.name),
            })
            .collect()
    }

    fn generate_recommendations(categories: &[SecurityCategory]) -> Vec<SecurityRecommendation> {
        categories
            .iter()
            .filter(|c| c.score < 70)
            .map(|c| {
                let (action, priority) = match c.name.as_str() {
                    "권한 보안" => (
                        "관리자 권한을 가진 역할 수를 줄이고, @everyone 권한을 검토하세요",
                        Priority::High,
                    ),
                    "조정 설정" => (
                        "인증 레벨을 높이고 2단계 인증을 활성화하세요",
                        Priority::High,
                    ),
                    "봇 보안" => (
                        "불필요한 봇을 제거하고 봇 권한을 최소화하세요",
                        Priority::Medium,
                    ),
                    _ => ("해당 설정을 검토하고 개선하세요", Priority::Medium),
                };
                SecurityRecommendation {
                    category: c.name.clone(),
                    action: action.into(),
                    priority,
                }
            })
            .collect()
    }

    fn solution_for(name: &str) -> String {
        if name.contains("@everyone") {
            "서버 설정 → 역할 → @everyone → 위험한 권한들을 비활성화하세요".into()
        } else if name.contains("2단계 인증") {
            "서버 설정 → 조정 → 관리 작업에 2단계 인증 요구를 활성화하세요".into()
        } else if name.contains("인증 레벨") {
            "서버 설정 → 조정 → 인증 레벨을 '중간' 이상으로 설정하세요".into()
        } else {
            "해당 설정을 검토하고 보안 모범 사례에 따라 수정하세요".into()
        }
    }

    // ── Report formatting ──────────────────────────────────────────

    pub fn format_report(report: &ServerSecurityReport) -> Vec<String> {
        let mut parts = Vec::new();

        // Part 1: overview
        let level_emoji = match report.security_level {
            SecurityLevel::Excellent => "🟢",
            SecurityLevel::Good => "🔵",
            SecurityLevel::Average => "🟡",
            SecurityLevel::Poor => "🟠",
            SecurityLevel::Critical => "🔴",
        };

        let mut p1 = format!(
            "🛡️ **{}** 서버 보안 리포트\n📅 검사 시간: {}\n\n{} **전체 보안 점수: {}/100** ({:?})\n\n📊 **카테고리별 점수**\n",
            report.server_name,
            report.check_timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            level_emoji,
            report.overall_score,
            report.security_level,
        );

        for cat in &report.categories {
            let emoji = if cat.score >= 80 {
                "✅"
            } else if cat.score >= 60 {
                "⚠️"
            } else {
                "❌"
            };
            p1.push_str(&format!("{emoji} {}: {}/100\n", cat.name, cat.score));
        }
        parts.push(p1);

        // Part 2: issues & recommendations
        if !report.critical_issues.is_empty() || !report.recommendations.is_empty() {
            let mut p2 = String::new();
            if !report.critical_issues.is_empty() {
                p2.push_str("🚨 **치명적 보안 이슈**\n");
                for (i, issue) in report.critical_issues.iter().enumerate() {
                    p2.push_str(&format!(
                        "{}. **{}**\n   해결: {}\n\n",
                        i + 1,
                        issue.title,
                        issue.solution,
                    ));
                }
            }
            if !report.recommendations.is_empty() {
                p2.push_str("💡 **개선 권장사항**\n");
                for rec in &report.recommendations {
                    let emoji = match rec.priority {
                        Priority::Immediate => "🔥",
                        Priority::High => "⚠️",
                        Priority::Medium => "📋",
                        Priority::Low => "💡",
                    };
                    p2.push_str(&format!("{emoji} {}\n", rec.action));
                }
            }
            parts.push(p2);
        }

        parts
    }
}
