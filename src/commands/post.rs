use std::time::Duration;

use crate::commands::{Context, Error, autocomplete_uma_name};
use crate::umasheet;
use serenity::all::{GuildId, MessageBuilder, ReactionType};
use url::Url;

async fn resolve_mentions(
    ctx: &Context<'_>,
    guild_id: GuildId,
    usernames: Vec<String>,
) -> Vec<String> {
    let mut mentions = Vec::new();

    for username in usernames {
        if let Ok(members) = guild_id
            .search_members(ctx.http(), &username, Some(1))
            .await
        {
            if let Some(member) = members.first() {
                if member.user.name == username {
                    mentions.push(format!("<@{}>", member.user.id));
                }
            }
        }
    }

    mentions
}

fn get_platform_from_url(url: &Url) -> &'static str {
    let host = url.host_str().unwrap_or("");
    if host.contains("twitter") || host.contains("x.com") {
        "Twitter"
    } else if host.contains("instagram") {
        "Instagram"
    } else if host.contains("reddit") {
        "Reddit"
    } else if host.contains("threads") {
        "Threads"
    } else if host.contains("pixiv") {
        "Pixiv"
    } else if host.contains("bsky") || host.contains("bluesky") {
        "Bluesky"
    } else if host.contains("bilibili") {
        "Bilibili"
    } else {
        "Source"
    }
}

#[poise::command(slash_command, prefix_command, guild_only)]
pub async fn post(
    ctx: Context<'_>,
    #[description = "Umamusume name"]
    #[autocomplete = "autocomplete_uma_name"]
    umamusume: String,
    #[description = "Link to post"] post: String,
    #[description = "Ping followers"] ping: Option<bool>,
) -> Result<(), Error> {
    let url = Url::parse(&post).map_err(|_| format!("`{}` is an invalid link", post))?;
    let platform = get_platform_from_url(&url);

    let guild_id = ctx.guild_id().ok_or("Cannot find guild")?;

    let followers = umasheet::get_followers(&umamusume).await?;
    let mentions = resolve_mentions(&ctx, guild_id, followers).await;

    let mentions_text = if mentions.is_empty() {
        "No one is following this uma.".to_string()
    } else {
        mentions.join(" ")
    };

    let mut content = MessageBuilder::new();
    content.push(format!(
        "[{} • {}](https://fixembed.app/embed?url={})\n",
        platform, umamusume, post
    ));
    if ping.unwrap_or(true) {
        content.push(format!("{}\n", mentions_text));
    }

    let reply = ctx.reply(content.build()).await?;
    let message = reply.into_message().await?;
    message
        .react(&ctx.http(), ReactionType::Unicode("❤️".to_string()))
        .await?;

    let author_id = ctx.author().id;
    let serenity_ctx = ctx.serenity_context().clone();

    tokio::spawn(async move {
        loop {
            if let Some(reaction) = message
                .await_reaction(&serenity_ctx.shard)
                .timeout(Duration::from_mins(5))
                .author_id(author_id)
                .await
            {
                if reaction.emoji == ReactionType::Unicode("❌".to_string()) {
                    let _ = message.delete(&serenity_ctx.http).await;
                    break;
                }
            }
        }
    });

    Ok(())
}
