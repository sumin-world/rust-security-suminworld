use chrono::{DateTime, Utc};
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::id::{ChannelId, UserId};
use serenity::prelude::*;

/// Send an embed to the given log channel (if set).
pub async fn log_embed(ctx: &Context, channel: Option<ChannelId>, title: &str, desc: &str) {
    if let Some(ch) = channel {
        let embed = CreateEmbed::new()
            .title(title)
            .description(desc)
            .timestamp(Utc::now());
        let message = CreateMessage::new().embed(embed);
        let _ = ch.send_message(&ctx.http, message).await;
    }
}

/// How many days since the user's Discord account was created.
#[allow(dead_code)]
pub fn account_age_days(ctx: &Context, user_id: UserId) -> Option<i64> {
    let user = ctx.cache.user(user_id)?;
    let created_dt: DateTime<Utc> = DateTime::from_timestamp(user.created_at().timestamp(), 0)?;
    Some((Utc::now() - created_dt).num_days())
}
