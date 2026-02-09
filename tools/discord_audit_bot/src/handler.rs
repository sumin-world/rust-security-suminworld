use crate::helpers::log_embed;
use crate::models::*;
use crate::scanner::SecurityScanner;
use crate::STATE;

use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateEmbedFooter, CreateMessage},
    model::{
        channel::Message,
        guild::{ExplicitContentFilter, Member, MfaLevel, VerificationLevel},
        prelude::{OnlineStatus, Presence},
    },
    prelude::*,
};

pub struct Handler;

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

        match msg.content.as_str() {
            "!ping" => {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "🏓 Pong! 봇이 살아있어요!")
                    .await;
            }
            "!안전" => self.cmd_info(&ctx, &msg).await,
            "!체크리스트" => self.cmd_checklist(&ctx, &msg).await,
            "!내짝" => self.cmd_my_buddy(&ctx, &msg).await,
            "!빠른스캔" => self.cmd_quick_scan(&ctx, &msg).await,
            "!스캔기록" => self.cmd_scan_history(&ctx, &msg).await,
            "!서버점검" => self.cmd_server_guide(&ctx, &msg).await,
            "!챌린지" => self.cmd_challenge(&ctx, &msg).await,
            "!실시간점검" => self.cmd_realtime(&ctx, &msg).await,
            "!도움말" | "!help" => self.cmd_help(&ctx, &msg).await,
            _ => {}
        }

        // Commands with arguments
        if msg.content.starts_with("!짝매칭") {
            self.cmd_pair(&ctx, &msg).await;
        }
        if msg.content.starts_with("!스캔") || msg.content.starts_with("!서버스캔") {
            // Avoid double-trigger with !스캔기록
            if msg.content != "!스캔기록" {
                self.cmd_full_scan(&ctx, &msg).await;
            }
        }
    }

    async fn presence_update(&self, ctx: Context, new_data: Presence) {
        if new_data.status == OnlineStatus::Online {
            let user_id = new_data.user.id;
            let display = ctx
                .cache
                .user(user_id)
                .map(|u| u.name.clone())
                .unwrap_or_else(|| format!("<@{}>", user_id.get()));

            let msg = format!("🔔 {display} 님이 온라인으로 전환했습니다!");
            let log_ch = STATE.read().await.log_channel;
            log_embed(&ctx, log_ch, "Presence Update", &msg).await;
        }
    }
}

// ── Command implementations ────────────────────────────────────────

impl Handler {
    async fn cmd_info(&self, ctx: &Context, msg: &Message) {
        let embed = CreateEmbed::new()
            .title("🔒 보안 감사 봇")
            .description("상호 보안 감사 시스템이 준비되었습니다!")
            .field(
                "기능",
                "• 실제 서버 보안 스캔\n• 보안 체크리스트\n• 상호 감사 시스템",
                false,
            )
            .color(0x00ff00);
        let _ = msg
            .channel_id
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await;
    }

    async fn cmd_checklist(&self, ctx: &Context, msg: &Message) {
        let items = [
            "2단계 인증 활성화 확인",
            "브라우저 확장 프로그램 권한 검토",
            "소셜미디어 공개 범위 설정",
            "자동 업데이트 활성화 상태",
            "VPN 사용 여부",
            "비밀번호 관리자 사용",
            "공공 Wi-Fi 사용 주의",
            "개인정보 백업 상태",
        ];
        let text: String = items
            .iter()
            .enumerate()
            .map(|(i, item)| format!("{}. {item}\n", i + 1))
            .collect();

        let embed = CreateEmbed::new()
            .title("🛡️ 보안 체크리스트")
            .description(text)
            .footer(CreateEmbedFooter::new(
                "각 항목을 확인하고 보안을 강화하세요!",
            ))
            .color(0x3498db);
        let _ = msg
            .channel_id
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await;
    }

    async fn cmd_pair(&self, ctx: &Context, msg: &Message) {
        if msg.mentions.is_empty() {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    "사용법: `!짝매칭 @사용자`\n예시: `!짝매칭 @친구`",
                )
                .await;
            return;
        }
        let mentioned = &msg.mentions[0];
        let mut state = STATE.write().await;
        let added = state.audit.add_pair(msg.author.id, mentioned.id);
        let response = if added {
            format!(
                "🤝 {}님과 {}님이 상호 보안 감사 짝이 되었습니다!",
                msg.author.name, mentioned.name
            )
        } else {
            "이미 등록된 짝입니다!".into()
        };
        let _ = msg.channel_id.say(&ctx.http, response).await;
    }

    async fn cmd_my_buddy(&self, ctx: &Context, msg: &Message) {
        let state = STATE.read().await;
        let pairs: Vec<_> = state
            .audit
            .buddy_pairs
            .iter()
            .filter(|(a, b)| *a == msg.author.id || *b == msg.author.id)
            .collect();

        let response = if pairs.is_empty() {
            "아직 짝이 없습니다. `!짝매칭 @사용자`로 짝을 만드세요!".into()
        } else {
            let mut text = "👥 나의 보안 감사 짝들:\n".to_string();
            for (a, b) in pairs {
                let partner = if *a == msg.author.id { b } else { a };
                text.push_str(&format!("• <@{}>\n", partner.get()));
            }
            text
        };
        let _ = msg.channel_id.say(&ctx.http, response).await;
    }

    async fn cmd_full_scan(&self, ctx: &Context, msg: &Message) {
        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "❌ 이 명령어는 서버에서만 사용 가능합니다.")
                    .await;
                return;
            }
        };

        // Clone guild out of cache ref to avoid holding CacheRef across await
        let guild_owned = { ctx.cache.guild(guild_id).map(|g| g.clone()) };
        let guild_owned = match guild_owned {
            Some(g) => g,
            None => {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "❌ 서버 정보를 가져올 수 없습니다.")
                    .await;
                return;
            }
        };

        let _ = msg
            .channel_id
            .say(
                &ctx.http,
                "🔍 서버 보안 감사를 시작합니다... (약 10초 소요)",
            )
            .await;

        let report = SecurityScanner::perform_security_audit(ctx, &guild_owned).await;

        {
            let mut state = STATE.write().await;
            state
                .security_reports
                .entry(guild_id.get())
                .or_default()
                .push(report.clone());
        }

        for part in SecurityScanner::format_report(&report) {
            let _ = msg.channel_id.say(&ctx.http, part).await;
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }

    async fn cmd_quick_scan(&self, ctx: &Context, msg: &Message) {
        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "❌ 서버에서만 사용 가능합니다.")
                    .await;
                return;
            }
        };

        let extracted = ctx.cache.guild(guild_id).map(|g| {
            let total = g.members.len();
            let bots = g.members.values().filter(|m| m.user.bot).count();
            let admins = g
                .roles
                .values()
                .filter(|r| r.permissions.administrator())
                .count();
            (
                g.name.clone(),
                g.verification_level,
                g.mfa_level,
                g.explicit_content_filter,
                total,
                bots,
                admins,
            )
        });

        let (name, verification, mfa, filter, total, bots, admins) = match extracted {
            Some(t) => t,
            None => {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "❌ 서버 정보를 가져올 수 없습니다.")
                    .await;
                return;
            }
        };

        let verification_status = match verification {
            VerificationLevel::None => "❌ 없음 (위험)",
            VerificationLevel::Low => "⚠️ 낮음",
            VerificationLevel::Medium => "✅ 보통",
            VerificationLevel::High => "✅ 높음",
            VerificationLevel::Higher => "✅ 매우 높음",
            _ => "❓ 알 수 없음",
        };
        let mfa_status = match mfa {
            MfaLevel::None => "❌ 비활성화",
            MfaLevel::Elevated => "✅ 활성화",
            _ => "❓ 알 수 없음",
        };
        let filter_status = match filter {
            ExplicitContentFilter::None => "❌ 비활성화",
            ExplicitContentFilter::WithoutRole => "⚠️ 부분적",
            ExplicitContentFilter::All => "✅ 전체",
            _ => "❓ 알 수 없음",
        };

        let bot_ratio = if total > 0 { (bots * 100) / total } else { 0 };
        let mut risk = 0u8;
        if matches!(verification, VerificationLevel::None) {
            risk += 1;
        }
        if matches!(mfa, MfaLevel::None) {
            risk += 1;
        }
        if bot_ratio > 30 {
            risk += 1;
        }
        if admins > 5 {
            risk += 1;
        }

        let risk_level = match risk {
            0 => "🟢 낮음",
            1 => "🟡 보통",
            2 => "🟠 높음",
            _ => "🔴 매우 높음",
        };

        let report = format!(
            "⚡ **{name}** 빠른 보안 점검\n\n\
             🔐 인증 레벨: {verification_status}\n\
             🛡️ 2단계 인증: {mfa_status}\n\
             🔒 콘텐츠 필터: {filter_status}\n\n\
             👥 총 멤버: {total}명\n\
             🤖 봇: {bots}개 ({bot_ratio}%)\n\
             ⚡ 관리자 역할: {admins}개\n\n\
             📊 위험도: {risk_level}\n\n\
             💡 상세한 분석을 원하시면 `!스캔`을 사용하세요."
        );
        let _ = msg.channel_id.say(&ctx.http, report).await;
    }

    async fn cmd_scan_history(&self, ctx: &Context, msg: &Message) {
        let guild_id = match msg.guild_id {
            Some(id) => id.get(),
            None => {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "❌ 이 명령어는 서버에서만 사용 가능합니다.")
                    .await;
                return;
            }
        };

        let state = STATE.read().await;
        let reports = state.security_reports.get(&guild_id);

        match reports {
            Some(reports) if !reports.is_empty() => {
                let mut history = "📊 **보안 감사 이력**\n\n".to_string();
                for (i, report) in reports.iter().rev().take(5).enumerate() {
                    let emoji = match report.security_level {
                        SecurityLevel::Excellent => "🟢",
                        SecurityLevel::Good => "🔵",
                        SecurityLevel::Average => "🟡",
                        SecurityLevel::Poor => "🟠",
                        SecurityLevel::Critical => "🔴",
                    };
                    history.push_str(&format!(
                        "{}. {emoji} **{}점** ({:?})\n   📅 {}\n\n",
                        i + 1,
                        report.overall_score,
                        report.security_level,
                        report.check_timestamp.format("%Y-%m-%d %H:%M"),
                    ));
                }
                let _ = msg.channel_id.say(&ctx.http, history).await;
            }
            _ => {
                let _ = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        "📋 아직 보안 감사 기록이 없습니다. `!스캔`으로 첫 번째 감사를 시작하세요.",
                    )
                    .await;
            }
        }
    }

    async fn cmd_server_guide(&self, ctx: &Context, msg: &Message) {
        let embed = CreateEmbed::new()
            .title("🔍 서버 보안 점검 가이드")
            .description("친구들과 함께 서버 보안을 점검해보세요!")
            .field(
                "1단계: 스크린샷 준비",
                "• 서버 설정 → 개요 페이지\n• 서버 설정 → 역할 → @everyone 권한\n• 서버 설정 → 감사 로그",
                false,
            )
            .field(
                "2단계: 점검 포인트",
                "• @everyone 권한 (관리자/킥/밴 권한 있으면 위험!)\n• 봇 역할 권한 (최소 권한 원칙)\n• 채널별 권한 설정\n• 감사 로그 활성화 여부",
                false,
            )
            .field(
                "3단계: 점수 매기기",
                "• 안전: 80-100점 🟢\n• 주의: 60-79점 🟡\n• 위험: 0-59점 🔴",
                false,
            )
            .footer(CreateEmbedFooter::new("이제 `!스캔` 명령어로 자동 분석도 가능합니다!"))
            .color(0xe74c3c);
        let _ = msg
            .channel_id
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await;
    }

    async fn cmd_challenge(&self, ctx: &Context, msg: &Message) {
        let embed = CreateEmbed::new()
            .title("🏆 이번 주 보안 챌린지")
            .description("친구들과 경쟁하며 보안을 강화하세요!")
            .field(
                "참여 방법",
                "1. `!스캔` - 서버 보안 점수 확인\n2. `!빠른스캔` - 간단한 점검\n3. `!스캔기록` - 개선 추이 확인",
                false,
            )
            .field(
                "이번 주 미션",
                "• 서버 보안 점수 80점 이상 달성\n• @everyone 권한 정리\n• 2단계 인증 활성화\n• 봇 권한 최소화",
                false,
            )
            .field(
                "보너스 점수",
                "• 친구 도와주기 (+5점)\n• 보안 팁 공유 (+3점)\n• 정기적인 점검 (+10점)",
                false,
            )
            .color(0xf39c12);
        let _ = msg
            .channel_id
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await;
    }

    async fn cmd_realtime(&self, ctx: &Context, msg: &Message) {
        let embed = CreateEmbed::new()
            .title("🎥 실시간 보안 점검")
            .description("음성채팅에서 화면공유로 즉석 점검!")
            .field(
                "준비물",
                "• 음성채팅 참가\n• 화면공유 준비\n• 점검할 설정 화면",
                false,
            )
            .field(
                "점검 순서",
                "1️⃣ `!스캔` 으로 자동 분석 먼저\n2️⃣ 화면공유로 설정 보여주기\n3️⃣ 친구들과 함께 개선하기\n4️⃣ `!스캔`으로 점수 확인",
                false,
            )
            .field(
                "점검 중 할 일",
                "• \"어? 저기 위험해!\" 🚨\n• \"그건 이렇게 고쳐!\" 💡\n• \"와 점수 올랐다!\" 🏆",
                false,
            )
            .footer(CreateEmbedFooter::new("이제 자동 스캔과 수동 점검을 함께 활용하세요!"))
            .color(0x9b59b6);
        let _ = msg
            .channel_id
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await;
    }

    async fn cmd_help(&self, ctx: &Context, msg: &Message) {
        let embed = CreateEmbed::new()
            .title("🛡️ 보안 감사 봇 명령어")
            .description("사용 가능한 모든 명령어들")
            .field(
                "🔍 자동 스캔 명령어",
                "`!스캔` - 전체 서버 보안 분석\n`!빠른스캔` - 간단한 보안 점검\n`!스캔기록` - 감사 기록 확인",
                false,
            )
            .field(
                "🤝 상호 감사 명령어",
                "`!짝매칭 @사용자` - 감사 짝 만들기\n`!내짝` - 현재 짝 확인\n`!체크리스트` - 보안 체크리스트",
                false,
            )
            .field(
                "🎮 게임화 명령어",
                "`!서버점검` - 수동 점검 가이드\n`!챌린지` - 주간 보안 챌린지\n`!실시간점검` - 화면공유 점검법",
                false,
            )
            .field(
                "ℹ️ 기타",
                "`!ping` - 봇 상태 확인\n`!안전` - 봇 소개\n`!도움말` - 이 메뉴",
                false,
            )
            .footer(CreateEmbedFooter::new("자동 스캔과 상호 감사로 서버를 안전하게!"))
            .color(0x3498db);
        let _ = msg
            .channel_id
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await;
    }
}
